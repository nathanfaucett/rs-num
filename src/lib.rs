use std::ops::*;


pub mod abs;
pub use abs::Abs;

pub mod pi;
pub use pi::PI;

pub mod from_primitive;
pub use from_primitive::FromPrimitive;

pub mod min;
pub use min::Min;

pub mod max;
pub use max::Max;

pub mod signum;
pub use signum::Signum;

pub mod one;
pub use one::One;

pub mod zero;
pub use zero::Zero;

pub mod sqrt;
pub use sqrt::Sqrt;

pub mod trig;
pub use trig::Trig;

pub trait Num:
    Copy + One + Zero + Sqrt
    + Min + Max + Signum
    + Abs
    + PI
    + FromPrimitive
    + Trig
    + PartialEq
    + PartialOrd
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
    + PI
    + FromPrimitive
    + Trig
    + PartialEq
    + PartialOrd
    + Add<T, Output = T>
    + Mul<T, Output = T>
    + Sub<T, Output = T>
    + Div<T, Output = T>
    + Rem<T, Output = T>
    + Neg<Output = T> {}
