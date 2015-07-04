pub trait Abs {
    fn abs(self) -> Self;
}

macro_rules! trait_abs {
    ($t:ident) => (
        impl Abs for $t {
            #[inline(always)]
            fn abs(self) -> $t { if self > 0 {self} else {-self} }
        }
    );
}

macro_rules! trait_abs_unsigned {
    ($t:ident) => (
        impl Abs for $t {
            #[inline(always)]
            fn abs(self) -> $t { if self > 0 {self} else {0} }
        }
    );
}

trait_abs_unsigned!(usize);
trait_abs_unsigned!(u8);
trait_abs_unsigned!(u16);
trait_abs_unsigned!(u32);
trait_abs_unsigned!(u64);

trait_abs!(isize);
trait_abs!(i8);
trait_abs!(i16);
trait_abs!(i32);
trait_abs!(i64);

impl Abs for f32 {
    #[inline(always)]
    fn abs(self) -> f32 { if self > 0.0f32 {self} else {-self}  }
}
impl Abs for f64 {
    #[inline(always)]
    fn abs(self) -> f64 { if self > 0.0f64 {self} else {-self}  }
}

#[test]
fn abs() {
    assert_eq!((-1).abs(), 1);
    assert_eq!((-1.0).abs(), 1.0);
    assert_eq!((2).abs(), 2);
}
