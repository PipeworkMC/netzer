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
use std::io::{ self, Write, Read };
#[cfg(feature = "smol")]
use smol::io::{
    AsyncWrite, AsyncWriteExt,
    AsyncRead, AsyncReadExt
};


#[non_exhaustive]
pub struct BigEndian;
impl Protocol for BigEndian { }

#[non_exhaustive]
pub struct LittleEndian;
impl Protocol for LittleEndian { }


macro_rules! impl_netencode_for_numeric {
    ( $ty:ty $(,)? ) => {

        impl NetEncode<BigEndian> for $ty {
            type Error = io::Error;
            fn encode<W : Write>(&self, mut writer : W) -> Result<(), Self::Error> {
                writer.write_all(&self.to_be_bytes())
            }
        }
        #[cfg(feature = "smol")]
        impl AsyncNetEncode<BigEndian> for $ty {
            async fn async_encode<W : AsyncWrite + Unpin>(&self, mut writer : W) -> Result<(), Self::Error> {
                writer.write_all(&self.to_be_bytes()).await
            }
        }

        impl NetDecode<BigEndian> for $ty {
            type Error = io::Error;
            fn decode<R : Read>(mut reader : R) -> Result<Self, Self::Error> {
                let mut buf = [0u8; size_of::<Self>()];
                reader.read_exact(&mut buf)?;
                Ok(Self::from_be_bytes(buf))
            }
        }
        #[cfg(feature = "smol")]
        impl AsyncNetDecode<BigEndian> for $ty {
            async fn async_decode<R : AsyncRead + Unpin>(mut reader : R) -> Result<Self, Self::Error> {
                let mut buf = [0u8; size_of::<Self>()];
                reader.read_exact(&mut buf).await?;
                Ok(Self::from_be_bytes(buf))
            }
        }

        impl NetEncode<LittleEndian> for $ty {
            type Error = io::Error;
            fn encode<W : Write>(&self, mut writer : W) -> Result<(), Self::Error> {
                writer.write_all(&self.to_le_bytes())
            }
        }
        #[cfg(feature = "smol")]
        impl AsyncNetEncode<LittleEndian> for $ty {
            async fn async_encode<W : AsyncWrite + Unpin>(&self, mut writer : W) -> Result<(), Self::Error> {
                writer.write_all(&self.to_le_bytes()).await
            }
        }

        impl NetDecode<LittleEndian> for $ty {
            type Error = io::Error;
            fn decode<R : Read>(mut reader : R) -> Result<Self, Self::Error> {
                let mut buf = [0u8; size_of::<Self>()];
                reader.read_exact(&mut buf)?;
                Ok(Self::from_le_bytes(buf))
            }
        }
        #[cfg(feature = "smol")]
        impl AsyncNetDecode<LittleEndian> for $ty {
            async fn async_decode<R : AsyncRead + Unpin>(mut reader : R) -> Result<Self, Self::Error> {
                let mut buf = [0u8; size_of::<Self>()];
                reader.read_exact(&mut buf).await?;
                Ok(Self::from_le_bytes(buf))
            }
        }

    }
}


impl_netencode_for_numeric!(u8);
impl_netencode_for_numeric!(i8);
impl_netencode_for_numeric!(u16);
impl_netencode_for_numeric!(i16);
impl_netencode_for_numeric!(u32);
impl_netencode_for_numeric!(i32);
impl_netencode_for_numeric!(u64);
impl_netencode_for_numeric!(i64);
impl_netencode_for_numeric!(u128);
impl_netencode_for_numeric!(i128);
impl_netencode_for_numeric!(f32);
impl_netencode_for_numeric!(f64);


impl NetEncode<BigEndian> for bool {
    type Error = io::Error;
    fn encode<W : Write>(&self, mut writer : W) -> Result<(), Self::Error> {
        writer.write_all(&[ if (*self) { 1u8 } else { 0u8 } ])
    }
}
#[cfg(feature = "smol")]
impl AsyncNetEncode<BigEndian> for bool {
    async fn async_encode<W : AsyncWrite + Unpin>(&self, mut writer : W) -> Result<(), Self::Error> {
        writer.write_all(&[ if (*self) { 1u8 } else { 0u8 } ]).await
    }
}

impl NetDecode<BigEndian> for bool {
    type Error = io::Error;
    fn decode<R : Read>(mut reader : R) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        Ok(buf[0] != 0)
    }
}
#[cfg(feature = "smol")]
impl AsyncNetDecode<BigEndian> for bool {
    async fn async_decode<R : AsyncRead + Unpin>(mut reader : R) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf).await?;
        Ok(buf[0] != 0)
    }
}

impl NetEncode<LittleEndian> for bool {
    type Error = io::Error;
    fn encode<W : Write>(&self, mut writer : W) -> Result<(), Self::Error> {
        writer.write_all(&[ if (*self) { 1u8 } else { 0u8 } ])
    }
}
#[cfg(feature = "smol")]
impl AsyncNetEncode<LittleEndian> for bool {
    async fn async_encode<W : AsyncWrite + Unpin>(&self, mut writer : W) -> Result<(), Self::Error> {
        writer.write_all(&[ if (*self) { 1u8 } else { 0u8 } ]).await
    }
}

impl NetDecode<LittleEndian> for bool {
    type Error = io::Error;
    fn decode<R : Read>(mut reader : R) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        Ok(buf[0] != 0)
    }
}
#[cfg(feature = "smol")]
impl AsyncNetDecode<LittleEndian> for bool {
    async fn async_decode<R : AsyncRead + Unpin>(mut reader : R) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf).await?;
        Ok(buf[0] != 0)
    }
}
