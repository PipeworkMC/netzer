use netzer::{
    NetEncode,
    numeric::BigEndian
};


#[derive(NetEncode)]
struct Hello {
    #[netzer(protocol = "BigEndian")]
    a : u64
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
