pub trait Signum {
    fn signum(self) -> Self;
}

impl Signum for u8 {
    #[inline(always)]
    fn signum(self) -> Self { if self > 0u8 {1u8} else {0u8} }
}

impl Signum for u16 {
    #[inline(always)]
    fn signum(self) -> Self { if self > 0u16 {1u16} else {0u16} }
}

impl Signum for u32 {
    #[inline(always)]
    fn signum(self) -> Self { if self > 0u32 {1u32} else {0u32} }
}

impl Signum for u64 {
    #[inline(always)]
    fn signum(self) -> Self { if self > 0u64 {1u64} else {0u64} }
}

impl Signum for i8 {
    #[inline(always)]
    fn signum(self) -> Self { if self > 0i8 {1i8} else if self < 0i8 {-1i8} else {0i8} }
}

impl Signum for i16 {
    #[inline(always)]
    fn signum(self) -> Self { if self > 0i16 {1i16} else if self < 0i16 {-1i16} else {0i16} }
}

impl Signum for i32 {
    #[inline(always)]
    fn signum(self) -> Self { if self > 0i32 {1i32} else if self < 0i32 {-1i32} else {0i32} }
}

impl Signum for i64 {
    #[inline(always)]
    fn signum(self) -> Self { if self > 0i64 {1i64} else if self < 0i64 {-1i64} else {0i64} }
}

impl Signum for f32 {
    #[inline(always)]
    fn signum(self) -> Self { if self > 0f32 {1f32} else if self < 0f32 {-1f32} else {0f32} }
}

impl Signum for f64 {
    #[inline(always)]
    fn signum(self) -> Self { if self > 0f64 {1f64} else if self < 0f64 {-1f64} else {0f64} }
}

#[test]
fn signum() {
    assert!((0u32).signum() == 0u32);
    assert!((1u32).signum() == 1u32);

    assert!((0i32).signum() == 0i32);
    assert!((-1i32).signum() == -1i32);
    assert!((1i32).signum() == 1i32);

    assert!((-1f32).signum() == -1f32);
    assert!((1f32).signum() == 1f32);
}
