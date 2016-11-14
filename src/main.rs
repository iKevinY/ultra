#[macro_use]
extern crate clap;
extern crate ultra;

use ultra::enigma::Enigma;

fn main() {
    let app = clap_app!(ultra =>
        (version: crate_version!())
        (@setting ArgRequiredElseHelp)
        (@setting ColoredHelp)
        (@arg ROTORS: --rotor -w +takes_value "Rotor order (default: \"123\")")
        (@arg KEY: --key -k +takes_value "Key settings (default: \"AAA\")")
        (@arg RING: --ring -r +takes_value "Ring settings (default: \"AAA\")")
        (@arg PLUGBOARD: --plugboard -p +takes_value "Plugboard settings (default: \"\")")
        (@arg MESSAGE: +required "The message to encrypt/decrypt")
    );

    let matches = app.get_matches();

    if let Some(msg) = matches.value_of("MESSAGE") {
        let rotors = match matches.value_of("ROTORS") {
            Some(rotors) => rotors,
            None => "123"
        };

        let key = match matches.value_of("KEY") {
            Some(key) => key,
            None => "AAA"
        };

        let ring = match matches.value_of("RING") {
            Some(ring) => ring,
            None => "AAA"
        };

        let plugboard = match matches.value_of("PLUGBOARD") {
            Some(plugboard) => plugboard,
            None => ""
        };

        let mut enigma = Enigma::new(rotors, key, ring, 'B', plugboard);
        println!("{}", enigma.encrypt(msg))
    }
}
