//! Cryptanalysis of the Engima in Rust.

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itertools;
extern crate ordered_float;
extern crate rand;
extern crate rayon;

mod decrypt;
mod constants;
mod enigma;
mod fitness;
mod plugboard;
mod reflector;
mod rotor;

pub use decrypt::decrypt;
pub use enigma::Enigma;


trait CharIndex {
    fn index(&self) -> usize;
}

impl CharIndex for char {
    fn index(&self) -> usize {
        *self as usize - 65
    }
}


trait ToChar {
    fn to_char(&self) -> char;
}

impl ToChar for usize {
    fn to_char(&self) -> char {
        ((*self % 26) as u8 + 65) as char
    }
}
