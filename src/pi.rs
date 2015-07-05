pub trait PI {
    fn half_pi() -> Self;
    fn pi() -> Self;
    fn two_pi() -> Self;
}

macro_rules! trait_pi {
    ($t:ident) => (
        impl PI for $t {
            #[inline(always)]
            fn half_pi() -> $t { 1 }
            fn pi() -> $t { 3 }
            fn two_pi() -> $t { 6 }
        }
    );
}

trait_pi!(usize);
trait_pi!(u8);
trait_pi!(u16);
trait_pi!(u32);
trait_pi!(u64);

trait_pi!(isize);
trait_pi!(i8);
trait_pi!(i16);
trait_pi!(i32);
trait_pi!(i64);

impl PI for f32 {
    #[inline(always)]
    fn half_pi() -> f32 { 1.57079632679489661923132169163975144f32 }
    fn pi() -> f32 { 3.14159265358979323846264338327950288f32 }
    fn two_pi() -> f32 { 6.28318530717958647692528676655900576f32 }
}

impl PI for f64 {
    #[inline(always)]
    fn half_pi() -> f64 { 1.57079632679489661923132169163975144f64 }
    fn pi() -> f64 { 3.14159265358979323846264338327950288f64 }
    fn two_pi() -> f64 { 6.28318530717958647692528676655900576f64 }
}
