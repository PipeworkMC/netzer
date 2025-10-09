use netzer::{
    NetEncode,
    numeric::BigEndian
};


#[derive(NetEncode)]
struct Hello {
    #[netzer(encode_with = encode_a)]
    a : u64
}
fn encode_a<W : std::io::Write>(a : &u64, mut w : W) -> Result<(), std::io::Error> {
    write!(w, "{}", a)
}


// #[derive(NetEncode)]
// #[netzer(ordinal, encode_as = "BigEndian", into = "u8")]
// #[repr(u8)]
// enum GameMode {
//     Survival(
//         #[netzer(encode_as = "BigEndian")]
//         u32
//     ) = 0,
//     Creative      = 1,
//     Adventure     = 2,
//     Spectator     = 3
// }

// #[derive(NetEncode)]
// #[netzer(nominal, encode_as = "Utf8<BigEndian>", into = "&str")]
// enum DimensionType {
//     #[netzer(rename = "minecraft:overworld")]
//     Overworld,
//     #[netzer(rename = "minecraft:the_nether")]
//     Nether,
//     #[netzer(rename = "minecraft:the_end")]
//     End
// }


fn main() {

}
