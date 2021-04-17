use criterion::{criterion_group, criterion_main, Criterion};

extern crate criterion;
extern crate ultra;

use ultra::decrypt;


fn bench_decrypt(c: &mut Criterion) {
    let msg = "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG";
    c.bench_function(
        "decrypt",
        |b| b.iter(|| decrypt(msg))
    );
}

criterion_group!{
    name = decrypt_benches;
    config = Criterion::default().sample_size(10);
    targets = bench_decrypt
}

criterion_main!(decrypt_benches);
