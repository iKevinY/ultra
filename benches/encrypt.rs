#![feature(test)]

extern crate ultra;
extern crate test;

use test::Bencher;
use ultra::enigma::Enigma;


#[bench]
fn bench_encrypt_char(b: &mut Bencher) {
    let mut enigma = Enigma::new(1, 2, 3, 'B', "AZ");
    b.iter(|| enigma.encrypt_char('A'));
}

#[bench]
fn bench_encrypt_msg(b: &mut Bencher) {
    let mut enigma = Enigma::new(1, 2, 3, 'B', "AB CD");
    b.iter(|| enigma.encrypt("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG"));
}
