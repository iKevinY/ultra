#[macro_use]
extern crate clap;
extern crate ultra;

use ultra::enigma::Enigma;

fn main() {
    let app = clap_app!(ultra =>
        (version: crate_version!())
        (@setting ArgRequiredElseHelp)
        (@setting ColoredHelp)
        (@arg MESSAGE: +required "The message to encrypt/decrypt")
    );

    let matches = app.get_matches();

    if let Some(msg) = matches.value_of("MESSAGE") {
        let mut enigma = Enigma::new("123", "AAA", "AAA", 'B', "");
        println!("{}", enigma.encrypt(msg))
    }
}
