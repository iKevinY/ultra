use std::fmt;

use itertools::Itertools;

use super::CharIndex;

#[derive(Clone, Debug)]
pub struct Plugboard {
    mapping: Vec<char>,
}

impl Plugboard {
    pub fn new(pairs: &str) -> Plugboard {
        let mut mapping: Vec<char> = ('A'..='Z').collect();

        for pair in pairs.split_whitespace() {
            let pair: Vec<char> = pair.chars().collect();
            let a = pair[0];
            let b = pair[1];

            mapping[a.index()] = b;
            mapping[b.index()] = a;
        }

        Plugboard { mapping }
    }

    pub fn map(&self, c: char) -> char {
        self.mapping[c.index()]
    }
}

impl fmt::Display for Plugboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let plugs: Vec<String> = self.mapping.iter().zip('A'..='Z')
            .filter(|(a, b)| a < &b)
            .map(|(a, b)| format!("{}{}", a, b))
            .collect();

        if plugs.is_empty() {
            write!(f, "<none>")
        } else {
            write!(f, "{}", plugs.iter().join(" "))
        }
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
