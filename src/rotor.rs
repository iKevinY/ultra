use std::ascii::AsciiExt;

pub struct Rotor<'a> {
    rotor: &'a [u8],
    length: usize,
}

impl<'a> Rotor<'a> {
    pub fn new(rotor: &'a str) -> Rotor<'a> {
        Rotor {
            rotor: rotor.as_bytes(),
            length: rotor.len(),
        }
    }

    pub fn substitute(&self, c: char) -> char {
        if !c.is_ascii() || !c.is_alphabetic() {
            return c;
        }

        let letter = c.to_ascii_uppercase();
        let offset = ((letter as u8) - ('A' as u8)) as usize;

        self.rotor[offset % self.length] as char
    }
}


#[cfg(test)]
mod tests {
    use super::Rotor;

    #[test]
    fn char_substitution() {
        let rotor = Rotor::new("XYZ");
        assert!(rotor.substitute('A') == 'X');
        assert!(rotor.substitute('b') == 'Y');
        assert!(rotor.substitute('F') == 'Z');
        assert!(rotor.substitute('!') == '!');
        assert!(rotor.substitute('é') == 'é');
    }
}
