use std::ascii::AsciiExt;
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Rotor {
    rotor: Vec<char>,
    length: usize,
    notches: HashSet<usize>,
    offset: usize,
}

impl Rotor {
    /// Creates a new `Rotor`.
    pub fn new(rotor: &str, notches: &str) -> Rotor {
        let rotor: Vec<char> = rotor.chars().collect();
        let rotor_len = rotor.len();

        Rotor {
            rotor: rotor,
            length: rotor_len,
            notches: HashSet::from_iter(notches.chars().map(|c| (c as usize) - 65)),
            offset: 0,
        }
    }

    /// Returns the substitution of a given character, dependent
    /// on the current offset of the rotor. For non-alphabetic
    /// characters, simply return the character itself.
    pub fn substitute(&self, c: char) -> char {
        if !c.is_ascii() || !c.is_alphabetic() {
            return c;
        }

        let letter = c.to_ascii_uppercase();
        let index = (letter as u8 as usize) - 65;

        self.rotor[(index + self.offset) % self.length]
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
}
