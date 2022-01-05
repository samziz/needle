#[inline]
pub fn high(n: u128) -> u64 {
    (n << 64) as u64
}

#[inline]
pub fn low(n: u128) -> u64 {
    n as u64
}