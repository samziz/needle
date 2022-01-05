use std::simd;

use crate::types::ID;
use super::int;


fn equals(id1: ID, id2: ID) -> bool {
    #[cfg(any(target_feature = "avx", target_feature = "sse"))]
    {
        simd::u64x2::from([int::low(id1), int::high(id1)])
            .eq(&simd::u64x2::from([int::low(id2), int::high(id2)]))
    }

    #[cfg(not(any(target_feature = "avx", target_feature = "sse")))]
    {
        id1 == id2
    }
}