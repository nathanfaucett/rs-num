use num::Num;


pub trait Unsigned:
    Num {}

impl<T> Unsigned for T where T:
    Num {}
