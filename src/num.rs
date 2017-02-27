use collections::string::ToString;
use core::ops::*;

use bounded::Bounded;
use to_primitive::ToPrimitive;
use from_primitive::FromPrimitive;
use round::Round;
use one::One;
use sqrt::Sqrt;
use trig::Trig;
use zero::Zero;


pub trait Num:
    Clone + One + Zero
    + Bounded
    + ToPrimitive
    + FromPrimitive
    + Trig
    + Sqrt
    + Round
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
    + RemAssign<Self>
{
    /// # Examples
    /// ~~~
    /// use num::Num;
    ///
    /// assert_eq!(50.min(&100), 50);
    /// assert_eq!(100.min(&50), 50);
    /// ~~~
    #[inline]
    fn min(&self, other: &Self) -> Self {
        if self < other {
            self.clone()
        } else {
            other.clone()
        }
    }
    /// # Examples
    /// ~~~
    /// use num::Num;
    ///
    /// assert_eq!(50.max(&100), 100);
    /// assert_eq!(100.max(&50), 100);
    /// ~~~
    #[inline]
    fn max(&self, other: &Self) -> Self {
        if self > other {
            self.clone()
        } else {
            other.clone()
        }
    }
    /// # Examples
    /// ~~~
    /// use num::Num;
    ///
    /// assert_eq!((-50).clamp(&0, &100), 0);
    /// assert_eq!(50.clamp(&0, &100), 50);
    /// assert_eq!(150.clamp(&0, &100), 100);
    /// ~~~
    #[inline]
    fn clamp(&self, min: &Self, max: &Self) -> Self {
        self.min(max).max(min)
    }
    /// # Examples
    /// ~~~
    /// use num::Num;
    ///
    /// assert_eq!((-0.5).clamp01(), 0.0);
    /// assert_eq!(0.5.clamp01(), 0.5);
    /// assert_eq!(1.50.clamp01(), 1.0);
    /// ~~~
    #[inline(always)]
    fn clamp01(&self) -> Self {
        self.clamp(&Zero::zero(), &One::one())
    }
}


impl<T> Num for T where T:
    Clone + One + Zero
    + Bounded
    + ToPrimitive
    + FromPrimitive
    + Trig
    + Sqrt
    + Round
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
