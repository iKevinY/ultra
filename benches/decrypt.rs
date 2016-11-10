#![feature(test)]

extern crate ultra;
extern crate test;

use test::Bencher;
use ultra::decrypt::qgram_score;


#[bench]
fn qgram_lookup(b: &mut Bencher) {
    b.iter(|| qgram_score("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG"));
}
