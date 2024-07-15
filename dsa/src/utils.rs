pub fn shift(ch: char, rot: u8, _ch: char) -> char {
    (((ch as u8 - _ch as u8) + rot) % 26 + _ch as u8) as char
}
