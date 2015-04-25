pub trait Max {
    fn max(self, other: Self) -> Self;
}

impl Max for u8 {
    #[inline(always)]
    fn max(self, other: Self) -> Self { if self > other { self } else { other } }
}

impl Max for u16 {
    #[inline(always)]
    fn max(self, other: Self) -> Self { if self > other { self } else { other } }
}

impl Max for u32 {
    #[inline(always)]
    fn max(self, other: Self) -> Self { if self > other { self } else { other } }
}

impl Max for u64 {
    #[inline(always)]
    fn max(self, other: Self) -> Self { if self > other { self } else { other } }
}

impl Max for i8 {
    #[inline(always)]
    fn max(self, other: Self) -> Self { if self > other { self } else { other } }
}

impl Max for i16 {
    #[inline(always)]
    fn max(self, other: Self) -> Self { if self > other { self } else { other } }
}

impl Max for i32 {
    #[inline(always)]
    fn max(self, other: Self) -> Self { if self > other { self } else { other } }
}

impl Max for i64 {
    #[inline(always)]
    fn max(self, other: Self) -> Self { if self > other { self } else { other } }
}

impl Max for f32 {
    #[inline(always)]
    fn max(self, other: Self) -> Self { if self > other { self } else { other } }
}

impl Max for f64 {
    #[inline(always)]
    fn max(self, other: Self) -> Self { if self > other { self } else { other } }
}

#[test]
fn max() {
    assert!((2u8).max(1u8) == 2u8);
    assert!((2u16).max(1u16) == 2u16);
    assert!((2u32).max(1u32) == 2u32);
    assert!((2u64).max(1u64) == 2u64);

    assert!((2i8).max(1i8) == 2i8);
    assert!((2i16).max(1i16) == 2i16);
    assert!((2i32).max(1i32) == 2i32);
    assert!((2i64).max(1i64) == 2i64);

    assert!((2f32).max(1f32) == 2f32);
    assert!((2f64).max(1f64) == 2f64);
}
