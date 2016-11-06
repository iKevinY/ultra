use util::map_char;

#[derive(Clone, Debug)]
pub struct Reflector {
    mapping: Vec<char>,
    length: usize,
}

impl Reflector {
    pub fn new(mapping: &str) -> Reflector {
        let mapping: Vec<char> = mapping.chars().collect();
        let length = mapping.len();

        Reflector {
            mapping: mapping,
            length: length,
        }
    }

    pub fn reflect(&self, c: char) -> char {
        map_char(c, &self.mapping, 0, self.length)
    }
}


#[cfg(test)]
mod tests {
    use super::Reflector;

    #[test]
    fn char_reflection() {
        let reflector = Reflector::new("XYZ");
        assert!(reflector.reflect('A') == 'X');
        assert!(reflector.reflect('b') == 'Y');
        assert!(reflector.reflect('!') == '!');
    }
}
