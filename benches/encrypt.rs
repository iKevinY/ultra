use criterion::{criterion_group, criterion_main, Criterion};

extern crate criterion;
extern crate ultra;

use ultra::Enigma;


fn bench_encrypt_char(c: &mut Criterion) {
    let mut enigma = Enigma::new("123", "AAA", "AAA", 'B', "AZ");
    c.bench_function(
        "encrypt_char",
        |b| b.iter(|| enigma.encrypt_char('A'))
    );
}

fn bench_encrypt_msg(c: &mut Criterion) {
    let mut enigma = Enigma::new("123", "AAA", "AAA", 'B', "AB CD");
    c.bench_function(
        "encrypt_msg",
        |b| b.iter(|| enigma.encrypt("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG"))
    );
}

criterion_group!(encrypt_benches, bench_encrypt_char, bench_encrypt_msg);
criterion_main!(encrypt_benches);
