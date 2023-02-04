#[derive(Clone, Debug, PartialEq)]
pub struct WzSound {
    playtime: i32,
    header: Vec<u8>,
    data: Vec<u8>,
}
