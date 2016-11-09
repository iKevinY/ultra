use std::iter::FromIterator;

#[derive(Clone, Debug)]
pub struct Plugboard {
    mapping: Vec<char>,
}

impl Plugboard {
    pub fn new(pairs: &str) -> Plugboard {
        let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut mapping = Vec::from_iter(alpha.chars());

        for pair in pairs.split_whitespace() {
            let a = pair.chars().nth(0).unwrap();
            let b = pair.chars().nth(1).unwrap();

            mapping[a as usize - 65] = b;
            mapping[b as usize - 65] = a;
        }

        Plugboard {
            mapping: mapping,
        }
    }

    pub fn map(&self, c: char) -> char {
        self.mapping[c as usize - 65]
    }
}


#[cfg(test)]
mod tests {
    use super::Plugboard;

    #[test]
    fn no_connections() {
        let plugboard = Plugboard::new("");
        assert_eq!(plugboard.map('A'), 'A');
    }

    #[test]
    fn single_connection() {
        let plugboard = Plugboard::new("AB");
        assert_eq!(plugboard.map('A'), 'B');
        assert_eq!(plugboard.map('B'), 'A');
        assert_eq!(plugboard.map('C'), 'C');
    }

    #[test]
    fn multiple_connections() {
        let plugboard = Plugboard::new("AB CD");
        assert_eq!(plugboard.map('A'), 'B');
        assert_eq!(plugboard.map('B'), 'A');
        assert_eq!(plugboard.map('C'), 'D');
        assert_eq!(plugboard.map('E'), 'E');
    }
}
