#![feature(test)]

extern crate ultra;
extern crate test;

use test::Bencher;
use ultra::Enigma;


#[bench]
fn encrypt_char(b: &mut Bencher) {
    let mut enigma = Enigma::new("123", "AAA", "AAA", 'B', "AZ");
    b.iter(|| enigma.encrypt_char('A'));
}

#[bench]
fn encrypt_msg(b: &mut Bencher) {
    let mut enigma = Enigma::new("123", "AAA", "AAA", 'B', "AB CD");
    b.iter(|| enigma.encrypt("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG"));
}
