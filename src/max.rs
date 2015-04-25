pub trait Max {
    fn max(self, other: Self) -> Self;
}

macro_rules! trait_max {
    ($t:ident) => (
        impl Max for $t {
            #[inline(always)]
            fn max(self, other: Self) -> Self { if self > other {self} else {other} }
        }
    );
}

trait_max!(usize);
trait_max!(u8);
trait_max!(u16);
trait_max!(u32);
trait_max!(u64);

trait_max!(isize);
trait_max!(i8);
trait_max!(i16);
trait_max!(i32);
trait_max!(i64);

trait_max!(f32);
trait_max!(f64);

#[test]
fn max() {
    assert_eq!((2u8).max(1u8), 2u8);
    assert_eq!((2u16).max(1u16), 2u16);
    assert_eq!((2u32).max(1u32), 2u32);
    assert_eq!((2u64).max(1u64), 2u64);

    assert_eq!((2i8).max(1i8), 2i8);
    assert_eq!((2i16).max(1i16), 2i16);
    assert_eq!((2i32).max(1i32), 2i32);
    assert_eq!((2i64).max(1i64), 2i64);

    assert_eq!((2f32).max(1f32), 2f32);
    assert_eq!((2f64).max(1f64), 2f64);
}
