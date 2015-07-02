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

macro_rules! trait_pi_float {
    ($t:ident) => (
        impl PI for $t {
            #[inline(always)]
            fn half_pi() -> $t { 1.57079632679489661923132169163975144 }
            fn pi() -> $t { 3.14159265358979323846264338327950288 }
            fn two_pi() -> $t { 6.28318530717958647692528676655900576 }
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

trait_pi_float!(f32);
trait_pi_float!(f64);
