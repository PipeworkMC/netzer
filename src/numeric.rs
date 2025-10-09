use crate::{
    Protocol,
    NetEncode,
    NetDecode
};
use std::io;
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
            async fn encode<W : AsyncWrite + Unpin>(&self, mut writer : W) -> Result<(), Self::Error> {
                writer.write_all(&self.to_be_bytes()).await
            }
        }
        impl NetDecode<BigEndian> for $ty {
            type Error = io::Error;
            async fn decode<R : AsyncRead + Unpin>(mut reader : R) -> Result<Self, Self::Error> {
                let mut buf = [0u8; size_of::<Self>()];
                reader.read_exact(&mut buf).await?;
                Ok(Self::from_be_bytes(buf))
            }
        }

        impl NetEncode<LittleEndian> for $ty {
            type Error = io::Error;
            async fn encode<W : AsyncWrite + Unpin>(&self, mut writer : W) -> Result<(), Self::Error> {
                writer.write_all(&self.to_le_bytes()).await
            }
        }
        impl NetDecode<LittleEndian> for $ty {
            type Error = io::Error;
            async fn decode<R : AsyncRead + Unpin>(mut reader : R) -> Result<Self, Self::Error> {
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
    async fn encode<W : AsyncWrite + Unpin>(&self, mut writer : W) -> Result<(), Self::Error> {
        writer.write_all(&[ if (*self) { 1u8 } else { 0u8 } ]).await
    }
}
impl NetDecode<BigEndian> for bool {
    type Error = io::Error;
    async fn decode<R : AsyncRead + Unpin>(mut reader : R) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf).await?;
        Ok(buf[0] != 0)
    }
}

impl NetEncode<LittleEndian> for bool {
    type Error = io::Error;
    async fn encode<W : AsyncWrite + Unpin>(&self, mut writer : W) -> Result<(), Self::Error> {
        writer.write_all(&[ if (*self) { 1u8 } else { 0u8 } ]).await
    }
}
impl NetDecode<LittleEndian> for bool {
    type Error = io::Error;
    async fn decode<R : AsyncRead + Unpin>(mut reader : R) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf).await?;
        Ok(buf[0] != 0)
    }
}
