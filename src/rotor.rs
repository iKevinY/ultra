use std::ascii::AsciiExt;
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Rotor {
    rotor: Vec<char>,
    inverse: Vec<char>,
    notches: HashSet<usize>,
    length: usize,
    offset: usize,
}

impl Rotor {
    /// Creates a new `Rotor`.
    pub fn new(rotor: &str, notches: &str) -> Rotor {
        let rotor: Vec<char> = rotor.chars().collect();
        let rotor_len = rotor.len();

        let inverse = {
            let mut inverse = vec!['A'; rotor_len];
            for (i, &c) in rotor.iter().enumerate() {
                let offset = ((c as u8) - 65u8) as usize;
                let letter = ((i as u8) + 65u8) as char;
                inverse[offset % rotor_len] = letter;
            }
            inverse
        };

        Rotor {
            rotor: rotor,
            inverse: inverse,
            length: rotor_len,
            notches: HashSet::from_iter(notches.chars().map(|c| (c as usize) - 65)),
            offset: 0,
        }
    }

    fn encode_char(&self, c: char, rotor: &Vec<char>) -> char {
        if !c.is_ascii() || !c.is_alphabetic() {
            return c;
        }

        let letter = c.to_ascii_uppercase();
        let index = (letter as u8 as usize) - 65;

        rotor[(index + self.offset) % self.length]
    }

    /// Returns the substitution of a given character, dependent
    /// on the current offset of the rotor. For non-alphabetic
    /// characters, simply return the character itself.
    pub fn substitute(&self, c: char) -> char {
        self.encode_char(c, &self.rotor)
    }

    /// Returns the substitution of a given character when run through
    /// the rotor in reverse (on the path back from the reflector).
    pub fn invert(&self, c: char) -> char {
        self.encode_char(c, &self.inverse)
    }

    /// Advances this rotor, returning `true` if the rotor adjacent to
    /// it should be advanced as well.
    pub fn advance(&mut self) -> bool {
        let advance = self.notches.contains(&self.offset);
        self.offset += 1 % self.length;
        return advance;
    }
}


#[cfg(test)]
mod tests {
    use super::Rotor;

    #[test]
    fn char_substitution() {
        let rotor = Rotor::new("XYZ", "A");
        assert!(rotor.substitute('A') == 'X');
        assert!(rotor.substitute('b') == 'Y');
        assert!(rotor.substitute('F') == 'Z');
        assert!(rotor.substitute('!') == '!');
        assert!(rotor.substitute('é') == 'é');
    }

    #[test]
    fn step_rotor() {
        let mut rotor = Rotor::new("ABC", "B");
        assert!(rotor.substitute('A') == 'A');

        // Step the rotor one position
        assert!(!rotor.advance());
        assert!(rotor.substitute('A') == 'B');

        // Moving from B to C should advance the next rotor
        assert!(rotor.advance());
        assert!(rotor.substitute('A') == 'C');
    }

    #[test]
    fn rotor_inverse() {
        // Rotor I of the Enigma
        let rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", "A");
        let inverse: String = rotor.inverse.into_iter().collect();
        assert!(&inverse == "UWYGADFPVZBECKMTHXSLRINQOJ");
    }
}
