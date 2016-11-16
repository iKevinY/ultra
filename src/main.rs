#[macro_use]
extern crate clap;
extern crate rand;
extern crate ultra;

use rand::Rng;

use ultra::enigma::Enigma;
use ultra::decrypt::decrypt;

fn main() {
    let app = clap_app!(ultra =>
        (version: crate_version!())
        (@setting ArgRequiredElseHelp)
        (@setting ColoredHelp)
        (@arg decrypt: --decrypt -d "Decrypt a given piece of ciphertext")
        (@arg randomize: --randomize -R "Encrypt a message with random Enigma settings")
        (@arg ROTORS: --rotor -w +takes_value "Rotor order (default: \"123\")")
        (@arg KEY: --key -k +takes_value "Key settings (default: \"AAA\")")
        (@arg RING: --ring -r +takes_value "Ring settings (default: \"AAA\")")
        (@arg PLUGBOARD: --plugboard -p +takes_value "Plugboard settings (default: \"\")")
        (@arg MESSAGE: +required "The message to encrypt/decrypt")
    );

    let matches = app.get_matches();
    let msg = matches.value_of("MESSAGE").unwrap();

    if matches.is_present("decrypt") {
        let (plaintext, key, ring, rotors) = decrypt(msg);
        println!("{}", plaintext);
        println!("(Key Setting: {}, Ring Setting: {}, Rotors: {})", key, ring, rotors);
    }

    else if matches.is_present("randomize") {
        let mut rng = rand::thread_rng();

        let rotors: String = {
            let mut rotor_pool: Vec<char> = "12345".chars().collect();
            let mut rotors: Vec<char> = Vec::with_capacity(3);

            for _ in 0..3 {
                let len = rotor_pool.len();
                rotors.push(rotor_pool.remove(rng.gen_range(0, len)));
            }

            rotors.into_iter().collect()
        };

        let mut key = String::with_capacity(3);
        let mut ring = String::with_capacity(3);

        for _ in 0..3 {
            key.push(rng.gen_range(b'A', b'Z' + 1) as char);
            ring.push(rng.gen_range(b'A', b'Z' + 1) as char);
        }

        let mut enigma = Enigma::new(&rotors, &key, &ring, 'B', "");
        println!("{}", enigma.encrypt(msg));
        println!("(Key Setting: {}, Ring Setting: {}, Rotors: {})", &key, &ring, &rotors);
    }

    else {
        let rotors = matches.value_of("ROTORS").unwrap_or("123");
        let key = matches.value_of("KEY").unwrap_or("AAA");
        let ring = matches.value_of("RING").unwrap_or("AAA");
        let plugboard = matches.value_of("PLUGBOARD").unwrap_or("");

        let mut enigma = Enigma::new(rotors, key, ring, 'B', plugboard);
        println!("{}", enigma.encrypt(msg));
    }
}
