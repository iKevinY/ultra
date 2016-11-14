use super::CharIndex;

use constants::REFLECTORS;

#[derive(Clone, Debug)]
pub struct Reflector {
    mapping: Vec<char>,
}

impl Reflector {
    /// Creates a new `Reflector` with a given 26-character mapping.
    pub fn new(mapping: &str) -> Reflector {
        Reflector {
            mapping: mapping.chars().collect(),
        }
    }

    pub fn from_enigma(reflector: char) -> Reflector {
        Reflector::new(REFLECTORS[reflector.index()])
    }

    pub fn reflect(&self, c: char) -> char {
        self.mapping[c.index()]
    }
}


#[cfg(test)]
mod tests {
    use super::Reflector;

    #[test]
    fn char_reflection() {
        let reflector = Reflector::new("EJMZALYXVBWFCRQUONTSPIKHGD");
        assert_eq!(reflector.reflect('A'), 'E');
        assert_eq!(reflector.reflect('B'), 'J');
    }
}
