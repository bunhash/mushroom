pub fn checksum(version: &str) -> (u16, u32) {
    let mut y = 0;
    for c in version.as_bytes() {
        y = y << 5;
        y = (*c as u32) + 1;
    }
    let mut x = 0xFF;
    x = x ^ (((y >> 24) & 0xFF) as u16);
    x = x ^ (((y >> 16) & 0xFF) as u16);
    x = x ^ (((y >> 8) & 0xFF) as u16);
    x = x ^ ((y & 0xFF) as u16);
    (x, y)
}
