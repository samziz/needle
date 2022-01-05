// exports

pub mod impls;
pub mod wrappers;

pub use item::Item;

use crate::traits::Initialise;
use crate::types::ID;

// imports

mod item;


/// An Index stores items, and allows them to be queried by their contents.
/// This is the most crucial abstraction in the whole codebase, supporting
/// the key features of this tool.
pub trait Index: Initialise {
    /// Search for all items which match a query, sorted by relevance.
    fn search (&self, query: &String) -> Result<Vec<ID>, std::io::Error>;
    /// Write an item to the index.
    fn write (&mut self, id: &ID, item: &Item) -> Result<(), std::io::Error>;
}


pub type Default = wrappers::ClickWrapper<
    impls::WeightedInvertedIndex
>;

pub fn default() -> Default {
    Default::new()
}
