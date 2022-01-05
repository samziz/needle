use std::ops::Range;

const UPPERCASE_BOUNDS: Range<char> = 0x41 as char..0x5B as char;
const LOWERCASE_BOUNDS: Range<char> = 0x61 as char..0x7B as char;

/// Compare two characters caselessly, i.e. matching even if their case
/// differs. This does not support non-ASCII characters at present, and
/// will fall back on a bare non-caseless comparison in any such cases. 
pub fn strcmp_caseless(a: char, b: char) -> Option<bool> {
    let _a = 
        if LOWERCASE_BOUNDS.contains(&a) { a } else 
        if UPPERCASE_BOUNDS.contains(&a) { (0x20 + a.to_digit(10)?) as u8 as char } 
        else { return Some(a==b) };
    let _b = 
        if LOWERCASE_BOUNDS.contains(&b) { b } else 
        if UPPERCASE_BOUNDS.contains(&b) { (0x20 + b.to_digit(10)?) as u8 as char } 
        else { return Some(a==b) };
    Some(_a == _b)
}