
pub trait IsZero{
    fn is_zero(&self) -> bool;
}

macro_rules! iz_int {
    ($ty: ty) => { impl IsZero for $ty{ fn is_zero(&self) -> bool { *self == 0 } } };
}

iz_int!(i8);
iz_int!(i16);
iz_int!(i32);
iz_int!(i64);
iz_int!(i128);

iz_int!(u8);
iz_int!(u16);
iz_int!(u32);
iz_int!(u64);
iz_int!(u128);

impl IsZero for f32 { fn is_zero(&self) -> bool { *self == 0. } }
impl IsZero for f64 { fn is_zero(&self) -> bool { *self == 0. } }
