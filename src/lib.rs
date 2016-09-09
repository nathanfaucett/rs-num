#![feature(collections, reflect_marker)]
#![no_std]


extern crate collections;

extern crate abs;
extern crate bounded;
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
use core::marker::Reflect;
use core::ops::*;


use abs::Abs;
use bounded::Bounded;
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
    + Bounded
    + ToPrimitive
    + FromPrimitive
    + Trig
    + Pi
    + PartialEq
    + PartialOrd
    + Round
    + ToString
    + Reflect

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
    + Bounded
    + ToPrimitive
    + FromPrimitive
    + Trig
    + Pi
    + PartialEq
    + PartialOrd
    + Round
    + ToString
    + Reflect

    + Add<T, Output = T>
    + Mul<T, Output = T>
    + Sub<T, Output = T>
    + Div<T, Output = T>
    + Rem<T, Output = T>
    + Neg<Output = T> {}
