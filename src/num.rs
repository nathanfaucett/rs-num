use collections::string::ToString;
use core::ops::*;
use core::cmp::Ord;

use abs::Abs;
use approx::Approx;
use bounded::Bounded;
use clamp::Clamp;
use to_degrees::ToDegrees;
use to_radians::ToRadians;
use to_primitive::ToPrimitive;
use from_primitive::FromPrimitive;
use max::Max;
use min::Min;
use one::One;
use round::Round;
use signum::Signum;
use sqrt::Sqrt;
use trig::Trig;
use zero::Zero;


pub trait Num:
    Copy + One + Zero
    + Min + Max + Signum
    + Abs
    + Approx
    + Bounded
    + Clamp
    + ToDegrees
    + ToRadians
    + ToPrimitive
    + FromPrimitive
    + Trig
    + Sqrt
    + Round
    + Ord
    + PartialEq
    + PartialOrd
    + ToString

    + Add<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Div<Self, Output = Self>
    + Rem<Self, Output = Self>

    + AddAssign<Self>
    + MulAssign<Self>
    + SubAssign<Self>
    + DivAssign<Self>
    + RemAssign<Self> {}


impl<T> Num for T where T:
    Copy + One + Zero
    + Min + Max + Signum
    + Abs
    + Approx
    + Bounded
    + Clamp
    + ToDegrees
    + ToRadians
    + ToPrimitive
    + FromPrimitive
    + Trig
    + Sqrt
    + Round
    + Ord
    + PartialEq
    + PartialOrd
    + ToString

    + Add<T, Output = T>
    + Mul<T, Output = T>
    + Sub<T, Output = T>
    + Div<T, Output = T>
    + Rem<T, Output = T>

    + AddAssign<T>
    + MulAssign<T>
    + SubAssign<T>
    + DivAssign<T>
    + RemAssign<T> {}
