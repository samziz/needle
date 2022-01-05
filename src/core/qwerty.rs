const QWERTY_KEYS: [[char; 10]; 3] = [
    ['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'],
    ['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ''],
    ['Z', 'X', 'C', 'V', 'B', 'B', 'N', 'M', '', ''],
];

fn key_distance(a: char, b: char) -> Option<usize> {
    let (a, b) = (a.to_uppercase().next()?, b.to_uppercase().next()?);
    let mut a_i = (0xFFusize, 0xFFusize);
    let mut b_i = (0xFFusize, 0xFFusize);
    let ref ks = QWERTY_KEYS;
    
    (0..QWERTY_KEYS.len()).find_map(|y|
        (0..QWERTY_KEYS[y].len()).find_map(|x| {
            if ks[y][x] == a {
                a_i = (x, y);
            }
            if ks[y][x] == b {
                b_i = (x, y);
            }
            if a_i != (0, 0) && b_i != (0, 0) {
                Some(
                    usize::abs_diff(a_i.0, b_i.0) +
                    usize::abs_diff(a_i.1, b_i.1)
                )
            } else {
                None
            }
        })
    )
}