pub trait Sqrt {
    fn sqrt(self) -> Self;
}

macro_rules! trait_sqrt_as {
    ($t:ident, $a:ident) => (
        impl Sqrt for $t {
            #[inline(always)]
            fn sqrt(self) -> Self {(self as $a).sqrt() as $t}
        }
    );
}

macro_rules! trait_sqrt {
    ($t:ident) => (
        impl Sqrt for $t {
            #[inline(always)]
            fn sqrt(self) -> Self {self.sqrt()}
        }
    );
}

trait_sqrt_as!(usize, f32);
trait_sqrt_as!(u8, f32);
trait_sqrt_as!(u16, f32);
trait_sqrt_as!(u32, f32);
trait_sqrt_as!(u64, f64);

trait_sqrt_as!(isize, f32);
trait_sqrt_as!(i8, f32);
trait_sqrt_as!(i16, f32);
trait_sqrt_as!(i32, f32);
trait_sqrt_as!(i64, f64);

trait_sqrt!(f32);
trait_sqrt!(f64);

#[test]
fn sqrt() {
    assert_eq!((4u8).sqrt(), 2u8);
    assert_eq!((4u16).sqrt(), 2u16);
    assert_eq!((4u32).sqrt(), 2u32);
    assert_eq!((4u64).sqrt(), 2u64);

    assert_eq!((4i8).sqrt(), 2i8);
    assert_eq!((4i16).sqrt(), 2i16);
    assert_eq!((4i32).sqrt(), 2i32);
    assert_eq!((4i64).sqrt(), 2i64);

    assert_eq!((4f32).sqrt(), 2f32);
    assert_eq!((4f64).sqrt(), 2f64);
}
