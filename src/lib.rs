#![no_std]


extern crate abs;
extern crate from_primitive;
extern crate max;
extern crate min;
extern crate one;
extern crate round;
extern crate signum;
extern crate sqrt;
extern crate trig;
extern crate zero;

use core::ops::*;

pub use abs::Abs;
pub use from_primitive::FromPrimitive;
pub use max::Max;
pub use min::Min;
pub use one::One;
pub use round::Round;
pub use signum::Signum;
pub use sqrt::Sqrt;
pub use trig::Trig;
pub use zero::Zero;

pub trait Num:
    Copy + One + Zero + Sqrt
    + Min + Max + Signum
    + Abs
    + FromPrimitive
    + Trig
    + PartialEq
    + PartialOrd
    + Round

    + Add<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Div<Self, Output = Self>
    + Rem<Self, Output = Self>
    + Neg<Output = Self> {}

impl<T> Num for T where T:
    Copy + One + Zero + Sqrt
    + Min + Max + Signum
    + Abs
    + FromPrimitive
    + Trig
    + PartialEq
    + PartialOrd
    + Round

    + Add<T, Output = T>
    + Mul<T, Output = T>
    + Sub<T, Output = T>
    + Div<T, Output = T>
    + Rem<T, Output = T>
    + Neg<Output = T> {}
