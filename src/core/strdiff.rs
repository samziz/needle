use itertools::Itertools;


pub fn strdiff (part: &String, whole: &String) -> usize {
    let mut d = 0usize;
    let mut i = 0usize;
    let (l1, l2) = (whole.len()-1, part.len()-1);
    let (part_ch, whole_ch) = (part.chars().collect_vec(), whole.chars().collect_vec());

    loop {
        if i == l1 { break d + (l2 - i) }
        if i == l2 { break d + (l1 - i) }
        // @todo calculate QWERTY distance, not simply inequality
        // @todo handle missing/added letters- means lookahead, ^ally higher time complexity
        if whole_ch[i] != part_ch[i] { d += 1; }
        i += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_diff_correctly () {
        assert_eq!(strdiff(
            &"enim magnis lacus".to_owned(),
            &"emin magnis lacus".to_owned(),
        ), 2);
    }
}