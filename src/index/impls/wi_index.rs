// imports

use std::collections::HashMap;
use std::iter::Iterator;

use itertools::Itertools;
use t1ha::{T1haBuildHasher, T1haHashMap};

use crate::core::strcmp::{strcmp, strcmp_part};
use crate::core::strdiff::strdiff;
use crate::traits::Initialise;
use crate::types::{ID, Word};
use super::super::Index;
use super::super::item::Item;


/// WeightedInvertedIndex is an inverted index which weights its
/// contents according to their relevance, and also according to
/// information such as click-and-query data.
#[derive(Debug)]
pub struct WeightedInvertedIndex <
    Hasher = T1haBuildHasher,
> {
    /// An inverted index mapping words to the IDs of all items
    /// which contain them. Words are lowercased; this decision
    /// was made to optimise comparisons, and can be deprecated
    /// and backfilled if we want to attach semantic meaning to
    /// the case of words in the index.
    index: HashMap<Word, Vec<ID>, Hasher>,
}

impl Initialise for WeightedInvertedIndex {
    fn new () -> Self {
        Self {
            index: T1haHashMap::default(),
        }
    }
}

impl WeightedInvertedIndex {
    /// Get an iterator over the matrix product of (a) each word in `qwords`
    /// and (b) each item in the index. This is the crucial 'iterand', over 
    /// which a higher-order 'fold' function can express almost any search
    /// algorithm. Note: This iterator is lazy, not computing until called.
    fn query_by_index (&'s self, query: &'q String) -> Result<
        impl Iterator<Item = (&'q str, (&'s String, &Vec<ID>))>,
        std::io::Error,
    > {
        // Split the query into an iterator over its words.
        let qword_it = query
            .split_ascii_whitespace()
            .unique();

        // Get an iterator over the index.
        let index_it = self.index
            .iter();

        // Compute the Cartesian product of the two above iterators: think matrix
        // multiplication, where [1,2,3]x[a,b] maps to [1,a],[1,b],[2,a],[2,b]...
        let q_by_w_iter = qword_it.cartesian_product(index_it);

        Ok(q_by_w_iter)
    }
}

impl Index for WeightedInvertedIndex {
    /// Search the index for the given query. Note that we use a slightly unusual
    /// definition of 'search': the index will return an iterator over every word
    /// in the query * every item in the index. (This is not expensive: iterators
    /// are lazy, and we would have to perform this search anyway.)
    fn search (&self, query: &'_ String) -> Result<Vec<ID>, std::io::Error> {
        // Lowercase the query. We store words in the index in lowercase: see the
        // comment on `Self.index` for a discussion of this behaviour.
        let query_lower = query.to_lowercase();

        let q_by_w_it = self.query_by_index(&query_lower)?;
        
        fn match_exact (qword: &str, iword: &String) -> bool {
            strcmp(&qword.to_string(), iword)
        }

        fn match_part (qword: &str, iword: &String) -> bool {
            strcmp_part(&qword.to_string(), iword)
        }

        fn match_typo (qword: &str, iword: &String) -> bool {
            strdiff(&qword.to_string(), iword) < (qword.len() / 4)
        }

        let ids = q_by_w_it
            .filter(|(qname, (iname,..))| 
                match_exact(qname, iname) ||
                match_part(qname, iname) ||
                match_typo(qname, iname)
            )
            .map(|(..,(..,ids))| ids)
            .flatten()
            .unique()
            .cloned()
            .collect();

        Ok(ids)
    }

    fn write (&mut self, &id: &ID, item: &Item) -> Result<(), std::io::Error> {
        item
            .iter()
            .for_each(|(_, value)|
                value
                    .to_lowercase()
                    .split_ascii_whitespace()
                    .unique()
                    .for_each(|word| {
                        let word = word.to_owned();
                        if let Some(vec) = self.index.get_mut(&word) {
                            if !vec.contains(&id) { vec.push(id) };
                        } else {
                            self.index.insert(word, Vec::from([id]));
                        }
                    })
            );

        Ok(())
    }
}

impl crate::storage::DiskStorage for WeightedInvertedIndex {
    fn deserialize <R> (&mut self, src: R) -> Result<(), std::io::Error>
        where R: std::io::Read {
        let de = serde_json::Deserializer::from_reader(src);
        let mut stream = de.into_iter::<(Word, Vec<ID>)>();
        
        loop {
            match stream.next() {
                Some(s) => match s {
                    Ok((word, ids)) => { self.index.insert(word, ids); },
                    Err(e) => break Err(e.into())
                },
                None => break Ok(()),
            }
        }
    }

    fn serialize <W> (&self, dst: W) -> Result<(), std::io::Error>
        where W: std::io::Write {
        use serde::{Serializer, ser::SerializeTuple};

        let ser = serde_json::Serializer::new(dst);
        let mut seq = ser.serialize_seq(Some(self.index.len()))?;
        for (iword, ids) in self.index {
            seq.serialize_element(&(iword, ids))?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn reads_back_simple_write () {
        let mut index = WeightedInvertedIndex::new();

        let id = 0u128;
        let doc = Item::mock();

        let w_result = index.write(&id, &doc);
        assert_matches!(w_result, Ok(_));

        let query = "world".to_owned();
        let r_result = index.search(&query);
        assert_matches!(r_result, Ok(_));

        let entries = r_result.unwrap();
        assert_eq!(entries.len(), 1);
    }
}