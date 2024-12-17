pub trait Scaler {}

// signed integers
impl Scaler for i8 {}
impl Scaler for i16 {}
impl Scaler for i32 {}
impl Scaler for i64 {}

// unsigned integers
impl Scaler for u8 {}
impl Scaler for u16 {}
impl Scaler for u32 {}
impl Scaler for u64 {}

// floats
impl Scaler for f32 {}
impl Scaler for f64 {}