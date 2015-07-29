pub trait Round {
    fn round(self) -> Self;
}

macro_rules! trait_round {
    ($t:ident) => (
        impl Round for $t {
            #[inline(always)]
            fn round(self) -> $t { self }
        }
    );
}

trait_round!(usize);
trait_round!(u8);
trait_round!(u16);
trait_round!(u32);
trait_round!(u64);

trait_round!(isize);
trait_round!(i8);
trait_round!(i16);
trait_round!(i32);
trait_round!(i64);

impl Round for f32 {
    #[inline(always)]
    fn round(self) -> f32 { self.round()  }
}
impl Round for f64 {
    #[inline(always)]
    fn round(self) -> f64 { self.round() }
}

#[test]
fn test_round() {
    assert_eq!((1.2f32).round(), 1f32);
    assert_eq!((2).round(), 2);
}
