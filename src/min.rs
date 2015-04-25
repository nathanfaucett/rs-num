pub trait Min {
    fn min(self, other: Self) -> Self;
}

impl Min for u8 {
    #[inline(always)]
    fn min(self, other: Self) -> Self { if self < other { self } else { other } }
}

impl Min for u16 {
    #[inline(always)]
    fn min(self, other: Self) -> Self { if self < other { self } else { other } }
}

impl Min for u32 {
    #[inline(always)]
    fn min(self, other: Self) -> Self { if self < other { self } else { other } }
}

impl Min for u64 {
    #[inline(always)]
    fn min(self, other: Self) -> Self { if self < other { self } else { other } }
}

impl Min for i8 {
    #[inline(always)]
    fn min(self, other: Self) -> Self { if self < other { self } else { other } }
}

impl Min for i16 {
    #[inline(always)]
    fn min(self, other: Self) -> Self { if self < other { self } else { other } }
}

impl Min for i32 {
    #[inline(always)]
    fn min(self, other: Self) -> Self { if self < other { self } else { other } }
}

impl Min for i64 {
    #[inline(always)]
    fn min(self, other: Self) -> Self { if self < other { self } else { other } }
}

impl Min for f32 {
    #[inline(always)]
    fn min(self, other: Self) -> Self { if self < other { self } else { other } }
}

impl Min for f64 {
    #[inline(always)]
    fn min(self, other: Self) -> Self { if self < other { self } else { other } }
}

#[test]
fn min() {
    assert!((2u8).min(1u8) == 1u8);
    assert!((2u16).min(1u16) == 1u16);
    assert!((2u32).min(1u32) == 1u32);
    assert!((2u64).min(1u64) == 1u64);

    assert!((2i8).min(1i8) == 1i8);
    assert!((2i16).min(1i16) == 1i16);
    assert!((2i32).min(1i32) == 1i32);
    assert!((2i64).min(1i64) == 1i64);

    assert!((2f32).min(1f32) == 1f32);
    assert!((2f64).min(1f64) == 1f64);
}
