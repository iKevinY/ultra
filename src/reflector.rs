#[derive(Clone, Debug)]
pub struct Reflector {
    mapping: Vec<char>,
}

impl Reflector {
    pub fn new(mapping: &str) -> Reflector {
        Reflector {
            mapping: mapping.chars().collect(),
        }
    }

    pub fn reflect(&self, c: char) -> char {
        self.mapping[c as usize - 65]
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
