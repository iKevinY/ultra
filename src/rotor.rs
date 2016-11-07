use std::collections::HashSet;
use std::iter::FromIterator;

use util::map_char;

#[derive(Clone, Debug)]
pub struct Rotor {
    mapping: Vec<char>,
    inverse: Vec<char>,
    notches: HashSet<usize>,
    offset: usize,
}

impl Rotor {
    /// Creates a new `Rotor`.
    pub fn new(mapping: &str, notches: &str) -> Rotor {
        let mapping: Vec<char> = mapping.chars().collect();

        if mapping.len() != 26 {
            panic!("Rotor mappings must be 26 characters long.");
        }

        let inverse = {
            let mut inverse = vec!['A'; 26];
            for (i, &c) in mapping.iter().enumerate() {
                let index = ((c as u8) - 65) as usize;
                let letter = ((i as u8) + 65) as char;
                inverse[index % 26] = letter;
            }
            inverse
        };

        Rotor {
            mapping: mapping,
            inverse: inverse,
            notches: HashSet::from_iter(notches.chars().map(|c| (c as usize) - 65)),
            offset: 0,
        }
    }

    /// Returns the substitution of a given character, dependent
    /// on the current offset of the rotor. For non-alphabetic
    /// characters, simply return the character itself.
    pub fn substitute(&self, c: char) -> char {
        map_char(c, &self.mapping, self.offset)
    }

    /// Returns the substitution of a given character when run through
    /// the rotor in reverse (on the path back from the reflector).
    pub fn invert(&self, c: char) -> char {
        map_char(c, &self.inverse, self.offset)
    }

    /// Advances this rotor, returning `true` if the rotor adjacent to
    /// it should be advanced as well.
    pub fn advance(&mut self) -> bool {
        let advance_next = self.notches.contains(&self.offset);
        self.offset = (self.offset + 1) % 26;
        return advance_next;
    }
}


#[cfg(test)]
mod tests {
    use super::Rotor;

    #[test]
    fn char_substitution() {
        let rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", "A");
        assert_eq!(rotor.substitute('A'), 'E');
        assert_eq!(rotor.substitute('b'), 'K');
        assert_eq!(rotor.substitute(' '), ' ');
        assert_eq!(rotor.substitute('é'), 'é');
    }

    #[test]
    fn step_rotor() {
        let mut rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", "B");
        assert_eq!(rotor.substitute('A'), 'E');

        // Step the rotor one position
        assert!(!rotor.advance());
        assert_eq!(rotor.offset, 1);
        assert_eq!(rotor.substitute('A'), 'K');

        // Moving from B to C should advance the next rotor
        assert!(rotor.advance());
        assert_eq!(rotor.offset, 2);
        assert_eq!(rotor.substitute('A'), 'M');
    }

    #[test]
    fn rotor_inverse() {
        // Rotor I of the Enigma
        let rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", "A");
        let inverse: String = rotor.inverse.into_iter().collect();
        assert_eq!(&inverse, "UWYGADFPVZBECKMTHXSLRINQOJ");
    }

    #[test]
    #[should_panic(expected = "Rotor mappings must be 26 characters long.")]
    fn invalid_rotor() {
        Rotor::new("ABC", "A");
    }
}
