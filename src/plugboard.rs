use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Plugboard {
    mapping: HashMap<char, char>,
}

impl Plugboard {
    pub fn new(pairs: &str) -> Plugboard {
        let mut mapping = HashMap::new();

        for pair in pairs.split_whitespace() {
            let a = pair.chars().nth(0).unwrap();
            let b = pair.chars().nth(1).unwrap();
            mapping.insert(a, b);
            mapping.insert(b, a);
        }

        Plugboard {
            mapping: mapping,
        }
    }

    pub fn map(&self, c: char) -> char {
        *self.mapping.get(&c).unwrap_or(&c)
    }
}


#[cfg(test)]
mod tests {
    use super::Plugboard;

    #[test]
    fn plugboard_map() {
        let plugboard = Plugboard::new("AB CD");
        assert_eq!(plugboard.map('A'), 'B');
        assert_eq!(plugboard.map('B'), 'A');
        assert_eq!(plugboard.map('C'), 'D');
        assert_eq!(plugboard.map('E'), 'E');
        assert_eq!(plugboard.map(' '), ' ');
    }
}
