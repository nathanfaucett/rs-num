pub trait Min {
    fn min(self, other: Self) -> Self;
}

macro_rules! trait_min {
    ($t:ident) => (
        impl Min for $t {
            #[inline(always)]
            fn min(self, other: Self) -> Self { if self < other {self} else {other} }
        }
    );
}

trait_min!(usize);
trait_min!(u8);
trait_min!(u16);
trait_min!(u32);
trait_min!(u64);

trait_min!(isize);
trait_min!(i8);
trait_min!(i16);
trait_min!(i32);
trait_min!(i64);

trait_min!(f32);
trait_min!(f64);

#[test]
fn min() {
    assert_eq!((2u8).min(1u8), 1u8);
    assert_eq!((2u16).min(1u16), 1u16);
    assert_eq!((2u32).min(1u32), 1u32);
    assert_eq!((2u64).min(1u64), 1u64);

    assert_eq!((2i8).min(1i8), 1i8);
    assert_eq!((2i16).min(1i16), 1i16);
    assert_eq!((2i32).min(1i32), 1i32);
    assert_eq!((2i64).min(1i64), 1i64);

    assert_eq!((2f32).min(1f32), 1f32);
    assert_eq!((2f64).min(1f64), 1f64);
}
