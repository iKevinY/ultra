# ultra [![Build Status][Travis Badge]][Build Status] [![crates.io][crates.io Badge]][crates.io] [![docs.rs][docs.rs Badge]][docs.rs] [![License][License Badge]](LICENSE)

`ultra` is a Rust implementation of the [Enigma machine] that includes the
ability to decrypt ciphertext.


## Installation

`ultra` can be installed from [crates.io] using Cargo:

```
$ cargo install ultra
```


## Usage

Encrypt a message with rotors `1-4-2`, key setting `D-O-G`, and ring setting `C-A-T`:

```bash
$ ultra --rotor=142 --key=DOG --ring=CAT "The quick brown fox jumps over the lazy dog."
Ntz ntqlz jmwll art bbnow wzqk keq ievk lzo.
```

Encrypt a message using random Enigma settings:

```bash
$ ultra --randomize "The quick brown fox jumps over the lazy dog."
Kxj mcwzf oqgmz pwr vnfqq iwhv wcr qqgt lgd.
Settings: Rotors: 5-2-3 / Key: A-A-G / Ring: N-W-Q / Plugs: CG EZ HW IJ MP TY
```

Attempt to decrypt a piece of ciphertext:

```bash
$ ultra --decrypt "$(cat ciphertext.txt)"
...
```

Decryption relies on a combination of [index of coincidence], bigram, and
quadgram frequencies to infer the original Enigma machine settings, and as a
result, it is quite likely that messages shorter than 500 characters will not
come anywhere close to being decrypted correctly.


## References

The original version of this project was based on [James Lyons'] articles about
the Enigma machine (see [this blog post] for a brief overview). As of version
0.6.0, the decryption algorithm was updated, inspired by [this Computerphile
video].


## License

`ultra` is licensed under the [MIT License](LICENSE).


[Travis Badge]: https://travis-ci.org/iKevinY/ultra.svg?branch=master
[Build Status]: https://travis-ci.org/iKevinY/ultra
[crates.io Badge]: https://img.shields.io/crates/v/ultra.svg
[crates.io]: https://crates.io/crates/ultra
[docs.rs Badge]: https://docs.rs/ultra/badge.svg
[docs.rs]: https://docs.rs/ultra
[License Badge]: https://img.shields.io/crates/l/ultra.svg

[Enigma machine]: https://en.wikipedia.org/wiki/Enigma_machine
[index of coincidence]: https://en.wikipedia.org/wiki/Index_of_coincidence
[this blog post]: http://kevinyap.ca/2017/04/breaking-the-enigma-code-with-rust/
[James Lyons']: http://practicalcryptography.com/ciphers/mechanical-era/enigma/
[this Computerphile video]: https://www.youtube.com/watch?v=RzWB5jL5RX0
