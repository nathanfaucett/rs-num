pub trait Trig {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
}

impl Trig for u8 {
    fn sin(self) -> u8 {(self as f32).sin() as u8}
    fn cos(self) -> u8 {(self as f32).cos() as u8}
    fn tan(self) -> u8 {(self as f32).tan() as u8}
}

impl Trig for u16 {
    fn sin(self) -> u16 {(self as f32).sin() as u16}
    fn cos(self) -> u16 {(self as f32).cos() as u16}
    fn tan(self) -> u16 {(self as f32).tan() as u16}
}

impl Trig for u32 {
    fn sin(self) -> u32 {(self as f32).sin() as u32}
    fn cos(self) -> u32 {(self as f32).cos() as u32}
    fn tan(self) -> u32 {(self as f32).tan() as u32}
}

impl Trig for u64 {
    fn sin(self) -> u64 {(self as f64).sin() as u64}
    fn cos(self) -> u64 {(self as f64).cos() as u64}
    fn tan(self) -> u64 {(self as f64).tan() as u64}
}

impl Trig for i8 {
    fn sin(self) -> i8 {(self as f32).sin() as i8}
    fn cos(self) -> i8 {(self as f32).cos() as i8}
    fn tan(self) -> i8 {(self as f32).tan() as i8}
}

impl Trig for i16 {
    fn sin(self) -> i16 {(self as f32).sin() as i16}
    fn cos(self) -> i16 {(self as f32).cos() as i16}
    fn tan(self) -> i16 {(self as f32).tan() as i16}
}

impl Trig for i32 {
    fn sin(self) -> i32 {(self as f32).sin() as i32}
    fn cos(self) -> i32 {(self as f32).cos() as i32}
    fn tan(self) -> i32 {(self as f32).tan() as i32}
}

impl Trig for i64 {
    fn sin(self) -> i64 {(self as f64).sin() as i64}
    fn cos(self) -> i64 {(self as f64).cos() as i64}
    fn tan(self) -> i64 {(self as f64).tan() as i64}
}

impl Trig for f32 {
    fn sin(self) -> f32 {self.sin()}
    fn cos(self) -> f32 {self.cos()}
    fn tan(self) -> f32 {self.tan()}
}

impl Trig for f64 {
    fn sin(self) -> f64 {self.sin()}
    fn cos(self) -> f64 {self.cos()}
    fn tan(self) -> f64 {self.tan()}
}

#[test]
fn trig() {
    assert!((0f32).sin() == 0f32);
}
