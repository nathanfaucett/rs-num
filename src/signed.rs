use core::ops::*;

use num::Num;


pub trait Signed:
    Num
    + Neg<Output = Self> {}

impl<T> Signed for T where T:
    Num
    + Neg<Output = T> {}
