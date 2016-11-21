#![feature(collections)]
#![no_std]


extern crate collections;

extern crate abs;
extern crate approx;
extern crate bounded;
extern crate to_degrees;
extern crate to_radians;
extern crate to_primitive;
extern crate from_primitive;
extern crate max;
extern crate min;
extern crate one;
extern crate pi;
extern crate round;
extern crate signum;
extern crate sqrt;
extern crate trig;
extern crate zero;


use collections::string::ToString;
use core::ops::*;

use abs::Abs;
use approx::Approx;
use bounded::Bounded;
use to_degrees::ToDegrees;
use to_radians::ToRadians;
use to_primitive::ToPrimitive;
use from_primitive::FromPrimitive;
use max::Max;
use min::Min;
use one::One;
use pi::Pi;
use round::Round;
use signum::Signum;
use sqrt::Sqrt;
use trig::Trig;
use zero::Zero;


pub trait Num:
    Copy + One + Zero + Sqrt
    + Min + Max + Signum
    + Abs
    + Approx
    + Bounded
    + ToDegrees
    + ToRadians
    + ToPrimitive
    + FromPrimitive
    + Trig
    + Pi
    + PartialEq
    + PartialOrd
    + Round
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
    Copy + One + Zero + Sqrt
    + Min + Max + Signum
    + Abs
    + Approx
    + Bounded
    + ToDegrees
    + ToRadians
    + ToPrimitive
    + FromPrimitive
    + Trig
    + Pi
    + PartialEq
    + PartialOrd
    + Round
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
