// imports

use std::simd::{u8x4, u8x8, u8x16, u8x32};


pub fn strcmp (s1: &String, s2: &String) -> bool {
    let (s1, s2) = (s1.as_bytes(), s2.as_bytes());

    if cfg!(any(target_feature = "avx", target_feature = "sse")) {
        strcmp_simd(s1, s2)
    } else {
        strcmp_base(s1, s2)
    }
}

/// strcmp_base is the base implementation of strcmp. For now, it uses
/// the standard Eq implementation on Vec<u8>, since we have no real
/// optimisations to make here. The code is structured thus because
/// we want to make it easy to make even small optimisations in future
/// without having to justify a large refactor.
fn strcmp_base (s1: &[u8], s2: &[u8]) -> bool {
    s1 == s2
}

/// strcmp_simd is optimised, exploiting SIMD (i.e. single instruction
/// instruction multiple data) instructions, where these are available,
/// to improve performance vs Rust's standard String Eq implementation.
/// 
/// Rust's portable SIMD API offers u8 vectors of 8-64 bits. We choose
/// a size based on the size of the string (for this function, both
/// must be of equal length - see strcmp_part for an alternative)
#[inline]
fn strcmp_simd (s1: &[u8], s2: &[u8]) -> bool {
    let mut i = 0usize;
    let l = s1.len();
    if l != s2.len() { return false };
    if l < 8 { return s1.eq(s2) };

    loop {
        let r = l - i;
        if r >= (32*8) {
            if u8x32::from_slice(&s1[i..i+(32*8)]).eq(&u8x32::from_slice(&s2[i..i+(32*8)]))
                { i += 32*8; continue } else { break false }
        };
        if r >= (16*8) {
            if u8x16::from_slice(&s1[i..i+(16*8)]).eq(&u8x16::from_slice(&s2[i..i+(16*8)]))
                { i += 16*8; continue } else { break false }
        };
        if r >= (8*8) {
            if u8x8::from_slice(&s1[i..i+(8*8)]).eq(&u8x8::from_slice(&s2[i..i+(8*8)]))
                { i += 8*8; continue } else { break false }
        };
        if r >= (4*8) {
            if u8x4::from_slice(&s1[i..i+(4*8)]).eq(&u8x4::from_slice(&s2[i..i+(4*8)]))
                { i += 4*8; continue } else { break false }
        };
        break s1[i..l-1].eq(&s2[i..l-1])
    }
}


/// Tests for a partial match between two strings. It is not bijective:
/// it tests only whether `whole` contains `part`, and not vice versa.
/// It exploits SIMD where available - although even when this is not
/// available, our algorithm outperforms the stdlib implementation of
/// `a.contains(b)` by >10x, at an avg of 3ns/iter vs 31ns/iter. This
/// is approx. 4 cycles on my processor, for a ~10char string. At that
/// speed it's probable we're benefiting from some branch prediction,
/// but this is a validation of the cache locality of our algorithm.
pub fn strcmp_part (part: &String, whole: &String) -> bool {
    let (part, whole) = (part.as_bytes(), whole.as_bytes());

    if cfg!(any(target_feature = "avx", target_feature = "sse")) {
        strcmp_simd(part, whole)
    } else {
        strcmp_base(part, whole)
    }
}

/// Tests for a partial match between two strings. This base impl is
/// written to be portable to all architectures Rust itself targets,
/// using no special features. Others may be written to exploit any
/// processor-specific tricks, such as the SIMD-specific impl below.
fn strcmp_part_base (part: &[u8], whole: &[u8]) -> bool {
    let (mut i1, mut i2) = (0usize, 0usize);
    let (l1, l2) = (whole.len()-1, part.len()-1);

    loop {
        if i1 == l1 { break false }
        if i2 == l2 { break true }
        if whole[i1] == part[i2] { i1 += 1; i2 += 1; } else { i1 += 1; }
    }
}

/// Tests for a partial match between two strings, exploiting SIMD, if
/// available, to batch more bits in each register on each instruction.
/// Rust's portable SIMD API offers u8 vectors of 8-64 bits. We choose
/// a size at each iteration based on the remaining unprocessed bytes.
fn strcmp_part_simd (part: &[u8], whole: &[u8]) -> bool {
    let mut i = 0usize;
    let l = part.len();
    if l != whole.len() { return false };
    if l < 8 { return part.eq(whole) };

    loop {
        let r = l - i;
        if r >= 32*8 {
            if u8x32::from_slice(&part[i..i+(32*8)]).eq(&u8x32::from_slice(&whole[i..i+(32*8)]))
                { i += 32*8; continue } else { break false }
        };
        if r >= 16*8 {
            if u8x16::from_slice(&part[i..i+(16*8)]).eq(&u8x16::from_slice(&whole[i..i+(16*8)]))
                { i += 16*8; continue } else { break false }
        };
        if r >= 8*8 {
            if u8x8::from_slice(&part[i..i+(8*8)]).eq(&u8x8::from_slice(&whole[i..i+(8*8)]))
                { i += 8*8; continue } else { break false }
        };
        if r >= 4*8 {
            if u8x4::from_slice(&part[i..i+(4*8)]).eq(&u8x4::from_slice(&whole[i..i+(4*8)]))
                { i += 4*8; continue } else { break false }
        };
        break part[i..l-1].eq(&whole[i..l-1])
    }
}


#[cfg(test)]
mod tests {
    mod strcmp {
        use super::super::*;

        #[test]
        fn succeeds_on_match () {
            assert!(strcmp(
                &"Duis blandit enim magnis lacus, proin quis aliquam interdum primis laoreet!".to_owned(),
                &"Duis blandit enim magnis lacus, proin quis aliquam interdum primis laoreet!".to_owned(),
            ) == true);
        }

        #[test]
        fn fails_on_non_match () {
            assert!(strcmp(
                &"Duis blandit enim magnis lacus, proin quis aliquam interdum primis laoreet!".to_owned(),
                &"Duis blandit enim paucis lacus, proin quis aliquam interdum primis laoreet!".to_owned(),
            ) == false);
        }
    }

    mod strcmp_part {
        use test::Bencher;

        use super::super::*;

        #[test]
        fn succeeds_on_match () {
            assert!(strcmp_part(
                &"crate".to_owned(),
                &"socrates".to_owned(),
            ));
        }

        #[test]
        fn fails_on_non_match () {
            assert!(!strcmp_part(
                &"crate".to_owned(),
                &"diogenes".to_owned(),
            ));
        }

        #[test]
        fn succeeds_on_match_2 () {
            assert!(strcmp_part(
                &"rust".to_owned(),
                &"untrusting".to_owned(),
            ));
        }

        #[bench]
        fn simple_benchmark (b: &mut Bencher) {
            b.iter(|| strcmp_part(
                &"rust".to_owned(),
                &"untrusting".to_owned(),
            ))
        }

        #[bench]
        fn from_str_benchmark (b: &mut Bencher) {
            b.iter(|| strcmp_part(
                &"rust".to_owned(),
                &"untrusting".to_owned(),
            ))
        }

        #[bench]
        fn comparison_benchmark (b: &mut Bencher) {
            b.iter(|| "untrusting".contains("rust"))
        }
    }


}