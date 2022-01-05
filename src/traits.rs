pub trait Initialise {
    fn new () -> Self;
}

/// Returns a shallow copy of Self, not mutating the original.
/// Any Copy (stack-allocated) fields are copied, whereas for
/// any Clone (heap-allocated) fields we simply copy the raw
/// pointer address. `shallow_copy()` on `{a: 0, b: [1,2,3]}`
/// is therefore copy-value for .a and copy-reference for .b.
pub trait ShallowCopy {
    fn shallow_copy (&self) -> Self;
}