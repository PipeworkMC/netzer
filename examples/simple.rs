use netzer::prelude::*;
use netzer::Result;


#[derive(NetEncode, NetDecode)]
pub struct InheritTest {
    #[netzer(format = "Inherit")]
    s : String
}


#[derive(NetEncode, NetDecode)]
pub struct Hello {
    #[netzer(encode_with = "encode_a", decode_with = "decode_a")]
    a : u64,
    #[netzer(format = "Leb128", convert = "VarInt<i64>", try_from)]
    b : i32,
    #[netzer(format = "Utf8<VarInt<u32>, Leb128>")]
    c : String
}
async fn encode_a<W : netzer::AsyncWrite>(a : &u64, mut w : W) -> Result {
    w.write_all(&(a.wrapping_add(1)).to_le_bytes()).await?;
    Ok(())
}
async fn decode_a<R : netzer::AsyncRead>(mut r : R) -> Result<u64> {
    let mut buf = [0u8; size_of::<u64>()];
    r.read_exact(&mut buf).await?;
    Ok(u64::from_le_bytes(buf).wrapping_sub(1))
}


#[derive(NetEncode, NetDecode)]
#[netzer(ordinal, format = "BigEndian")]
#[repr(u8)]
pub enum GameMode {
    Survival(
        #[netzer(format = "BigEndian")]
        u32
    ) = 0,
    Creative  = 1,
    Adventure = 2,
    Spectator = 3
}

#[derive(NetEncode, NetDecode)]
#[netzer(nominal, format = "Utf8<u16, BigEndian>", convert = "&str")]
pub enum DimensionType {
    #[netzer(rename = "minecraft:overworld")]
    Overworld,
    #[netzer(rename = "minecraft:the_nether")]
    Nether,
    #[netzer(rename = "minecraft:the_end")]
    End
}


fn main() {

}
