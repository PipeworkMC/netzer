use netzer::{
    NetEncode,
    numeric::BigEndian,
    string::Utf8,
    varint::{ VarInt, Leb128 },
    Result
};


#[derive(NetEncode)]
pub struct Hello {
    #[netzer(format_with = "encode_a")]
    a : u64,
    #[netzer(format = "Leb128", convert = "VarInt<i64>")]
    b : i32,
    #[netzer(format = "Utf8<VarInt<u32>, Leb128>")]
    c : String
}
async fn encode_a<W : netzer::AsyncWrite>(a : &u64, mut w : W) -> Result {
    write!(w, "{a}").await
}


#[derive(NetEncode)]
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

#[derive(NetEncode)]
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
