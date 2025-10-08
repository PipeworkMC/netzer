use netzer::{
    NetEncode,
    numeric::BigEndian
};


#[derive(NetEncode)]
struct A {
    #[netzer(encode_as = "BigEndian")]
    a : u64
}


fn main() {

}
