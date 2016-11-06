use std::ascii::AsciiExt;

pub fn map_char(c: char, mapping: &Vec<char>, offset: usize, length: usize) -> char {
    if !c.is_ascii() || !c.is_alphabetic() {
        return c;
    }

    let letter = c.to_ascii_uppercase();
    let index = (letter as usize) - 65;

    mapping[(index + offset) % length]
}
