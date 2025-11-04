use crate::{
    NetFormat,
    NetEncode,
    NetDecode,
    Result, Error
};
use core::marker::PhantomData;
use std::borrow::Cow;



pub struct LengthPrefixed<Len, LenF : NetFormat, T, F : NetFormat> {
    values  : Vec<T>,
    _marker : PhantomData<(Len, LenF, F,)>
}
impl<Len, LenF : NetFormat, T, F : NetFormat> NetFormat for LengthPrefixed<Len, LenF, T, F> { }


pub trait LengthPrefixedType {}

impl<T> LengthPrefixedType for Vec<T> {}

impl<T> LengthPrefixedType for Cow<'_, [T]>
where
    [T] : ToOwned
{}

impl<T, const N : usize> LengthPrefixedType for [T; N] {}


// impl<Len, LenF : NetFormat, T, F : NetFormat> TryFrom<LengthPrefixed<Len, LenF, T, F>> for Vec<T> {
//     fn from(value : LengthPrefixed<Len, LenF, T, F>) -> Self {
//         value.values
//     }
// }

// impl<Len, LenF : NetFormat, T, F : NetFormat> TryFrom<LengthPrefixed<Len, LenF, T, F>> for Cow<'_, [T]>
// where
//     [T] : ToOwned
// {
//     fn from(value : LengthPrefixed<Len, LenF, T, F>) -> Self {
//         Cow::Owned(value.values)
//     }
// }


// impl<I, Len, LenF : NetFormat, T : NetEncode<F>, F : NetFormat> NetEncode<LengthPrefixed<Len, LenF, T, F>> for I
// where
//     for<'l> &'l I                             : IntoIterator<Item = T>,
//     for<'l> <&'l I as IntoIterator>::IntoIter : ExactSizeIterator,
//     Len                                       : NetEncode<LenF> + TryFrom<usize>,
//     Error                                     : From<<Len as TryFrom<usize>>::Error>
// {
//     async fn encode<W : crate::AsyncWrite>(&self, mut writer : W) -> Result {
//         let mut iter = <&I as IntoIterator>::into_iter(self);
//         let     len  = iter.len();
//         Len::try_from(len)?
//             .encode(&mut writer).await?;
//         for _ in 0..len {
//             <<&I as IntoIterator>::Item as NetEncode<F>>
//                 ::encode(&iter.next().unwrap(), &mut writer)
//                 .await?;
//         }
//         Ok(())
//     }
// }

// impl<I, Len, LenF : NetFormat, T : NetDecode<F>, F : NetFormat> NetDecode<LengthPrefixed<Len, LenF, T, F>> for I
// where
//     for<'l> I     : FromCapacity<T>,
//             Len   : NetDecode<LenF> + TryInto<usize>,
//             Error : From<<Len as TryInto<usize>>::Error>
// {
//     async fn decode<R : crate::AsyncRead>(mut reader : R) -> Result<Self> {
//         let len = Len::decode(&mut reader).await?
//             .try_into()?;
//         let mut builder = <I as FromCapacity<T>>::with_capacity(len)?;
//         for _ in 0..len {
//             <I as FromCapacity<T>>::push(&mut builder,
//                 <T as NetDecode<F>>
//                     ::decode(&mut reader)
//                     .await?
//             )?;
//         }
//         Ok(<I as FromCapacity<T>>::finalise(builder))
//     }
// }
