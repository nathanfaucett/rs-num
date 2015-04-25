pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for u8 {
    fn sqrt(self) -> u8 {(self as f32).sqrt() as u8}
}

impl Sqrt for u16 {
    fn sqrt(self) -> u16 {(self as f32).sqrt() as u16}
}

impl Sqrt for u32 {
    fn sqrt(self) -> u32 {(self as f32).sqrt() as u32}
}

impl Sqrt for u64 {
    fn sqrt(self) -> u64 {(self as f32).sqrt() as u64}
}

impl Sqrt for i8 {
    fn sqrt(self) -> i8 {(self as f32).sqrt() as i8}
}

impl Sqrt for i16 {
    fn sqrt(self) -> i16 {(self as f32).sqrt() as i16}
}

impl Sqrt for i32 {
    fn sqrt(self) -> i32 {(self as f32).sqrt() as i32}
}

impl Sqrt for i64 {
    fn sqrt(self) -> i64 {(self as f64).sqrt() as i64}
}

impl Sqrt for f32 {
    fn sqrt(self) -> f32 {self.sqrt()}
}

impl Sqrt for f64 {
    fn sqrt(self) -> f64 {self.sqrt()}
}

#[test]
fn sqrt() {
    assert!((4u8).sqrt() == 2u8);
    assert!((4u16).sqrt() == 2u16);
    assert!((4u32).sqrt() == 2u32);
    assert!((4u64).sqrt() == 2u64);

    assert!((4i8).sqrt() == 2i8);
    assert!((4i16).sqrt() == 2i16);
    assert!((4i32).sqrt() == 2i32);
    assert!((4i64).sqrt() == 2i64);

    assert!((4f32).sqrt() == 2f32);
    assert!((4f64).sqrt() == 2f64);
}
