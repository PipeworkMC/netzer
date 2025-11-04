use crate::{
    NetFormat,
    NetEncode,
    NetDecode,
    Result, Error
};
use core::marker::PhantomData;
use std::borrow::Cow;
use smol::io::{
    AsyncWrite, AsyncWriteExt,
    AsyncRead, AsyncReadExt
};


pub struct Utf8<Len, LenF : NetFormat> {
    _marker : PhantomData<(Len, LenF,)>
}
impl<Len, LenF : NetFormat> NetFormat for Utf8<Len, LenF> { }


impl<Len, LenF : NetFormat> NetEncode<Utf8<Len, LenF>> for str
where
    Len   : NetEncode<LenF> + TryFrom<usize>,
    Error : From<<Len as TryFrom<usize>>::Error>
{
    async fn encode<W : AsyncWrite + Unpin>(&self, mut writer : W) -> Result {
        let b = self.as_bytes();
        Len::try_from(b.len())?
            .encode(&mut writer).await?;
        writer.write_all(b).await?;
        Ok(())
    }
}
impl<Len, LenF : NetFormat> NetEncode<Utf8<Len, LenF>> for Cow<'_, str>
where
    Len   : NetEncode<LenF> + TryFrom<usize>,
    Error : From<<Len as TryFrom<usize>>::Error>
{
    async fn encode<W : AsyncWrite + Unpin>(&self, writer : W) -> Result {
        <str as NetEncode<Utf8<Len, LenF>>>::encode(&**self, writer).await
    }
}
impl<Len, LenF : NetFormat> NetEncode<Utf8<Len, LenF>> for String
where
    Len   : NetEncode<LenF> + TryFrom<usize>,
    Error : From<<Len as TryFrom<usize>>::Error>
{
    async fn encode<W : AsyncWrite + Unpin>(&self, writer : W) -> Result {
        <str as NetEncode<Utf8<Len, LenF>>>::encode(&**self, writer).await
    }
}


impl<Len, LenF : NetFormat> NetDecode<Utf8<Len, LenF>> for String
where
    Len   : NetDecode<LenF> + TryInto<usize>,
    Error : From<<Len as TryInto<usize>>::Error>
{
    async fn decode<R : AsyncRead + Unpin>(mut reader : R) -> Result<Self> {
        let len = Len::decode(&mut reader).await?
            .try_into()?;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf).await?;
        Ok(String::from_utf8(buf)?)
    }
}
impl<Len, LenF : NetFormat> NetDecode<Utf8<Len, LenF>> for Cow<'_, str>
where
    Len   : NetDecode<LenF> + TryInto<usize>,
    Error : From<<Len as TryInto<usize>>::Error>
{
    async fn decode<R : AsyncRead + Unpin>(reader : R) -> Result<Self> {
        Ok(Cow::Owned(<String as NetDecode<Utf8<Len, LenF>>>::decode(reader).await?))
    }
}
