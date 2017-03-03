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

    + Add<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>

    + AddAssign
    + MulAssign
    + SubAssign
    + DivAssign
    + RemAssign
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
    #[inline(always)]
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
    /// # Examples
    /// ~~~
    /// use num::Num;
    ///
    /// assert_eq!(10.abs_diff(&15), 5);
    /// assert_eq!(15.abs_diff(&10), 5);
    /// ~~~
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        if self > other {
            self.clone() - other.clone()
        } else {
            other.clone() - self.clone()
        }
    }
}


impl<'a, 'b, T> Num for T where
    &'a T: Add<T, Output = T>
         + Mul<T, Output = T>
         + Sub<T, Output = T>
         + Div<T, Output = T>
         + Rem<T, Output = T>

         + Add<&'b T, Output = T>
         + Mul<&'b T, Output = T>
         + Sub<&'b T, Output = T>
         + Div<&'b T, Output = T>
         + Rem<&'b T, Output = T>,

    T: 'a + 'b + Clone + One + Zero
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

     + Add<&'b T, Output = T>
     + Mul<&'b T, Output = T>
     + Sub<&'b T, Output = T>
     + Div<&'b T, Output = T>
     + Rem<&'b T, Output = T>

     + AddAssign<T>
     + MulAssign<T>
     + SubAssign<T>
     + DivAssign<T>
     + RemAssign<T> {}
