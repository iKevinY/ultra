use rotor::Rotor;

pub struct Enigma<'a> {
    slow: Rotor<'a>,
    mid: Rotor<'a>,
    fast: Rotor<'a>,
}

impl<'a> Enigma<'a> {
    /// Creates a new `Enigma`.
    pub fn new(slow: Rotor<'a>, mid: Rotor<'a>, fast: Rotor<'a>) -> Enigma<'a> {
        Enigma {
            slow: slow,
            mid: mid,
            fast: fast,
        }
    }

    /// Advances the `fast` rotor of the `Enigma`, and also advances
    /// the `mid` and `slow` rotors if appropriate.
    pub fn advance(&mut self) {
        if self.fast.advance() {
            if self.mid.advance() {
                self.slow.advance();
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use rotor::Rotor;
    use super::Enigma;

    #[test]
    fn advance_enigma() {
        let rotor1 = Rotor::new("ABC", "A");
        let rotor2 = Rotor::new("DEF", "A");
        let rotor3 = Rotor::new("GHI", "A");

        let mut enigma = Enigma::new(rotor1, rotor2, rotor3);
        enigma.advance();
    }
}
