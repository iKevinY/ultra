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
(Rotors: 314, Key Setting: NHO, Ring Setting: VTB)
```

Decrypt the ciphertext generated in the previous example:

```bash
$ ultra --decrypt "Kxj mcwzf oqgmz pwr vnfqq iwhv wcr qqgt lgd."
The quick brown fox jumps over the lazy dog.
(Rotors: 314, Key Setting: SZO, Ring Setting: ALB)
```

Decryption relies on quadgram frequencies to infer the original Enigma
machine settings, and as a result, it is quite likely that short messages
will be decrypted incorrectly. Additionally, decryption will not work for
messages that were encrypted with any plugboard plugs active. See
[this blog post] for a brief overview of the decryption algorithm.


## References

This project's quadgram data and decryption algorithm is based on
[James Lyons'] articles about the Enigma machine.


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
[this blog post]: http://kevinyap.ca/2017/04/breaking-the-enigma-code-with-rust/
[James Lyons']: http://practicalcryptography.com/ciphers/mechanical-era/enigma/
