use crate::{
    Protocol,
    NetEncode,
    NetDecode
};
#[cfg(feature = "smol")]
use crate::{
    AsyncNetEncode,
    AsyncNetDecode
};
use core::ops::{ BitAnd, BitOr, Shl, Shr, Not };
use std::io::{ self, Write, Read };
#[cfg(feature = "smol")]
use smol::io::{
    AsyncWrite, AsyncWriteExt,
    AsyncRead, AsyncReadExt
};


pub const SEGMENT_BITS : u8 = 0b01111111;
pub const CONTINUE_BIT : u8 = 0b10000000;


#[non_exhaustive]
pub struct Leb128;
impl Protocol for Leb128 { }

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


pub struct VarInt<T>(pub T);

impl<T : Leb128VarIntType> NetEncode<Leb128> for VarInt<T> {
    type Error = io::Error;
    fn encode<W : Write>(&self, mut writer : W) -> Result<(), Self::Error> {
        let self_segment_bits = T::Raw::from_u8(SEGMENT_BITS);
        let self_continue_bit = T::Raw::from_u8(CONTINUE_BIT);
        let u8_max            = T::Raw::from_u8(u8::MAX);
        let mut v = T::to_raw(self.0);
        loop {
            if ((v & (! self_segment_bits)) == T::Raw::ZERO) {
                writer.write_all(&[T::Raw::to_u8(v & u8_max)])?;
                return Ok(());
            }
            writer.write_all(&[T::Raw::to_u8((v & self_segment_bits) | self_continue_bit)])?;
            v = T::Raw::from_unsigned(T::Raw::to_unsigned(v) >> 7);
        }
    }
}

#[cfg(feature = "smol")]
impl<T : Leb128VarIntType> AsyncNetEncode<Leb128> for VarInt<T> {
    async fn async_encode<W : AsyncWrite + Unpin>(&self, mut writer : W) -> Result<(), Self::Error> {
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
    type Error = Leb128DecodeError;
    fn decode<R : Read>(mut reader : R) -> Result<Self, Self::Error> {
        let max_shift = size_of::<T::Raw>() * 8;
        let mut v     = T::Raw::ZERO;
        let mut shift = 0;
        loop {
            let mut buf = [0u8; 1];
            reader.read_exact(&mut buf).map_err(Leb128DecodeError::Io)?;
            let b = buf[0];
            v = v | (T::Raw::from_u8(b & SEGMENT_BITS) << shift);
            if ((b & CONTINUE_BIT) == 0) { break; }
            shift += 7;
            if (shift > max_shift) { return Err(Leb128DecodeError::TooLong); }
        }
        Ok(Self(T::from_raw(v)))
    }
}

#[cfg(feature = "smol")]
impl<T : Leb128VarIntType> AsyncNetDecode<Leb128> for VarInt<T> {
    async fn async_decode<R : AsyncRead + Unpin>(mut reader : R) -> Result<Self, Self::Error> {
        let max_shift = size_of::<T::Raw>() * 8;
        let mut v     = T::Raw::ZERO;
        let mut shift = 0;
        loop {
            let mut buf = [0u8; 1];
            reader.read_exact(&mut buf).await.map_err(Leb128DecodeError::Io)?;
            let b = buf[0];
            v = v | (T::Raw::from_u8(b & SEGMENT_BITS) << shift);
            if ((b & CONTINUE_BIT) == 0) { break; }
            shift += 7;
            if (shift > max_shift) { return Err(Leb128DecodeError::TooLong); }
        }
        Ok(Self(T::from_raw(v)))
    }
}
