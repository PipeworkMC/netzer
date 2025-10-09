use crate::{
    NetFormat,
    NetEncode,
    NetDecode
};
use core::{
    error::Error as StdError,
    fmt::{ self, Display, Formatter },
    ops::{ BitAnd, BitOr, Shl, Shr, Not }
};
use std::io;
use smol::io::{
    AsyncWrite, AsyncWriteExt,
    AsyncRead, AsyncReadExt
};


pub const SEGMENT_BITS : u8 = 0b01111111;
pub const CONTINUE_BIT : u8 = 0b10000000;


#[non_exhaustive]
pub struct Leb128;
impl NetFormat for Leb128 { }

pub enum Leb128DecodeError {
    TooLong,
    Io(io::Error)
}

pub trait Leb128RawVarIntType
    : Copy
    + BitAnd<Self, Output = Self>
    + BitOr<Self, Output = Self>
    + Not<Output = Self>
    + PartialEq<Self>
    + Shl<usize, Output = Self>
{
    type Unsigned : Copy + Shr<usize, Output = Self::Unsigned>;
    const ZERO : Self;
    fn from_u8(v : u8) -> Self;
    fn to_u8(self) -> u8;
    fn from_unsigned(v : Self::Unsigned) -> Self;
    fn to_unsigned(self) -> Self::Unsigned;
}
impl Leb128RawVarIntType for i16 {
    type Unsigned = u16;
    const ZERO : Self = 0;
    #[inline(always)] fn from_u8(v : u8) -> Self { v as Self }
    #[inline(always)] fn to_u8(self) -> u8 { self as u8 }
    #[inline(always)] fn from_unsigned(v : Self::Unsigned) -> Self { v.cast_signed() }
    #[inline(always)] fn to_unsigned(self) -> Self::Unsigned { self.cast_unsigned() }
}
impl Leb128RawVarIntType for i32 {
    type Unsigned = u32;
    const ZERO : Self = 0;
    #[inline(always)] fn from_u8(v : u8) -> Self { v as Self }
    #[inline(always)] fn to_u8(self) -> u8 { self as u8 }
    #[inline(always)] fn from_unsigned(v : Self::Unsigned) -> Self { v.cast_signed() }
    #[inline(always)] fn to_unsigned(self) -> Self::Unsigned { self.cast_unsigned() }
}
impl Leb128RawVarIntType for i64 {
    type Unsigned = u64;
    const ZERO : Self = 0;
    #[inline(always)] fn from_u8(v : u8) -> Self { v as Self }
    #[inline(always)] fn to_u8(self) -> u8 { self as u8 }
    #[inline(always)] fn from_unsigned(v : Self::Unsigned) -> Self { v.cast_signed() }
    #[inline(always)] fn to_unsigned(self) -> Self::Unsigned { self.cast_unsigned() }
}
impl Leb128RawVarIntType for i128 {
    type Unsigned = u128;
    const ZERO : Self = 0;
    #[inline(always)] fn from_u8(v : u8) -> Self { v as Self }
    #[inline(always)] fn to_u8(self) -> u8 { self as u8 }
    #[inline(always)] fn from_unsigned(v : Self::Unsigned) -> Self { v.cast_signed() }
    #[inline(always)] fn to_unsigned(self) -> Self::Unsigned { self.cast_unsigned() }
}

pub trait Leb128VarIntType : Copy {
    type Raw : Leb128RawVarIntType;
    fn to_raw(self) -> Self::Raw;
    fn from_raw(v : Self::Raw) -> Self;
}
impl<T : Leb128RawVarIntType> Leb128VarIntType for T {
    type Raw = Self;
    #[inline(always)] fn to_raw(self) -> Self::Raw { self }
    #[inline(always)] fn from_raw(v : Self::Raw) -> Self { v }
}
impl Leb128VarIntType for u16 {
    type Raw = i16;
    #[inline(always)] fn to_raw(self) -> Self::Raw { self.cast_signed() }
    #[inline(always)] fn from_raw(v : Self::Raw) -> Self { v.cast_unsigned() }
}
impl Leb128VarIntType for u32 {
    type Raw = i32;
    #[inline(always)] fn to_raw(self) -> Self::Raw { self.cast_signed() }
    #[inline(always)] fn from_raw(v : Self::Raw) -> Self { v.cast_unsigned() }
}
impl Leb128VarIntType for u64 {
    type Raw = i64;
    #[inline(always)] fn to_raw(self) -> Self::Raw { self.cast_signed() }
    #[inline(always)] fn from_raw(v : Self::Raw) -> Self { v.cast_unsigned() }
}
impl Leb128VarIntType for u128 {
    type Raw = i128;
    #[inline(always)] fn to_raw(self) -> Self::Raw { self.cast_signed() }
    #[inline(always)] fn from_raw(v : Self::Raw) -> Self { v.cast_unsigned() }
}

#[derive(Debug)]
pub struct Leb128TooLong;
impl StdError for Leb128TooLong { }
impl Display for Leb128TooLong {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        write!(f, "leb128 too long")
    }
}


pub struct VarInt<T>(pub T);

impl<T : Leb128VarIntType> NetEncode<Leb128> for VarInt<T> {
    async fn encode<W : AsyncWrite + Unpin>(&self, mut writer : W) -> crate::Result {
        let self_segment_bits = T::Raw::from_u8(SEGMENT_BITS);
        let self_continue_bit = T::Raw::from_u8(CONTINUE_BIT);
        let u8_max            = T::Raw::from_u8(u8::MAX);
        let mut v = T::to_raw(self.0);
        loop {
            if ((v & (! self_segment_bits)) == T::Raw::ZERO) {
                writer.write_all(&[T::Raw::to_u8(v & u8_max)]).await?;
                return Ok(());
            }
            writer.write_all(&[T::Raw::to_u8((v & self_segment_bits) | self_continue_bit)]).await?;
            v = T::Raw::from_unsigned(T::Raw::to_unsigned(v) >> 7);
        }
    }
}

impl<T : Leb128VarIntType> NetDecode<Leb128> for VarInt<T> {
    async fn decode<R : AsyncRead + Unpin>(mut reader : R) -> crate::Result<Self> {
        let max_shift = size_of::<T::Raw>() * 8;
        let mut v     = T::Raw::ZERO;
        let mut shift = 0;
        loop {
            let mut buf = [0u8; 1];
            reader.read_exact(&mut buf).await?;
            let b = buf[0];
            v = v | (T::Raw::from_u8(b & SEGMENT_BITS) << shift);
            if ((b & CONTINUE_BIT) == 0) { break; }
            shift += 7;
            if (shift > max_shift) { return Err(Leb128TooLong.into()); }
        }
        Ok(Self(T::from_raw(v)))
    }
}


macro_rules! impl_from_for_varint {
    ( $from:ty, $into:ty $(,)? ) => {
        impl From<$from> for VarInt<$into> {
            fn from(value : $from) -> Self {
                Self(<$into as From<$from>>::from(value))
            }
        }
        impl From<VarInt<$from>> for $into {
            fn from(value : VarInt<$from>) -> $into {
                <$into as From<$from>>::from(value.0)
            }
        }
    };
}

impl_from_for_varint!(bool, u16);
impl_from_for_varint!(u8, u16);
impl_from_for_varint!(u16, u16);
impl_from_for_varint!(bool, u32);
impl_from_for_varint!(u8, u32);
impl_from_for_varint!(u16, u32);
impl_from_for_varint!(u32, u32);
impl_from_for_varint!(bool, u64);
impl_from_for_varint!(u8, u64);
impl_from_for_varint!(u16, u64);
impl_from_for_varint!(u32, u64);
impl_from_for_varint!(u64, u64);
impl_from_for_varint!(bool, u128);
impl_from_for_varint!(u8, u128);
impl_from_for_varint!(u16, u128);
impl_from_for_varint!(u32, u128);
impl_from_for_varint!(u64, u128);
impl_from_for_varint!(u128, u128);
impl_from_for_varint!(bool, i16);
impl_from_for_varint!(u8, i16);
impl_from_for_varint!(i8, i16);
impl_from_for_varint!(i16, i16);
impl_from_for_varint!(bool, i32);
impl_from_for_varint!(u8, i32);
impl_from_for_varint!(u16, i32);
impl_from_for_varint!(i8, i32);
impl_from_for_varint!(i16, i32);
impl_from_for_varint!(i32, i32);
impl_from_for_varint!(bool, i64);
impl_from_for_varint!(u8, i64);
impl_from_for_varint!(u16, i64);
impl_from_for_varint!(u32, i64);
impl_from_for_varint!(i8, i64);
impl_from_for_varint!(i16, i64);
impl_from_for_varint!(i32, i64);
impl_from_for_varint!(i64, i64);
impl_from_for_varint!(bool, i128);
impl_from_for_varint!(u8, i128);
impl_from_for_varint!(u16, i128);
impl_from_for_varint!(u32, i128);
impl_from_for_varint!(u64, i128);
impl_from_for_varint!(i8, i128);
impl_from_for_varint!(i16, i128);
impl_from_for_varint!(i32, i128);
impl_from_for_varint!(i64, i128);
impl_from_for_varint!(i128, i128);

macro_rules! impl_tryfrom_for_varint {
    ( $from:ty, $into:ty $(,)? ) => {
        impl TryFrom<$from> for VarInt<$into> {
            type Error = <$into as TryFrom<$from>>::Error;
            fn try_from(value : $from) -> Result<Self, Self::Error> {
                Ok(Self(<$into as TryFrom<$from>>::try_from(value)?))
            }
        }
        impl TryFrom<VarInt<$from>> for $into {
            type Error = <$into as TryFrom<$from>>::Error;
            fn try_from(value : VarInt<$from>) -> Result<$into, Self::Error> {
                <$into as TryFrom<$from>>::try_from(value.0)
            }
        }
    };
}
impl_tryfrom_for_varint!(u32, u16);
impl_tryfrom_for_varint!(u64, u16);
impl_tryfrom_for_varint!(u128, u16);
impl_tryfrom_for_varint!(usize, u16);
impl_tryfrom_for_varint!(u64, u32);
impl_tryfrom_for_varint!(u128, u32);
impl_tryfrom_for_varint!(usize, u32);
impl_tryfrom_for_varint!(u128, u64);
impl_tryfrom_for_varint!(usize, u64);
impl_tryfrom_for_varint!(usize, u128);
impl_tryfrom_for_varint!(u16, i16);
impl_tryfrom_for_varint!(u32, i16);
impl_tryfrom_for_varint!(u64, i16);
impl_tryfrom_for_varint!(u128, i16);
impl_tryfrom_for_varint!(i32, i16);
impl_tryfrom_for_varint!(i64, i16);
impl_tryfrom_for_varint!(i128, i16);
impl_tryfrom_for_varint!(isize, i16);
impl_tryfrom_for_varint!(u32, i32);
impl_tryfrom_for_varint!(u64, i32);
impl_tryfrom_for_varint!(u128, i32);
impl_tryfrom_for_varint!(i64, i32);
impl_tryfrom_for_varint!(i128, i32);
impl_tryfrom_for_varint!(isize, i32);
impl_tryfrom_for_varint!(u64, i64);
impl_tryfrom_for_varint!(u128, i64);
impl_tryfrom_for_varint!(i128, i64);
impl_tryfrom_for_varint!(isize, i64);
impl_tryfrom_for_varint!(u128, i128);
impl_tryfrom_for_varint!(isize, i128);
