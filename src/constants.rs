/// Enigma rotor and reflector information from Wikipedia:
/// https://en.wikipedia.org/wiki/Enigma_rotor_details

pub const ROTORS: &'static [&'static str; 8] = &[
    "EKMFLGDQVZNTOWYHXUSPAIBRCJ",
    "AJDKSIRUXBLHWTMCQGZNPYFVOE",
    "BDFHJLCPRTXVZNYEIWGAKMUSQO",
    "ESOVPZJAYQUIRHXLNFTGKDCMWB",
    "VZBRGITYUPSDNHLXAWMJQOFECK",
    "JPGVOUMFYQBENHZRDKASXLICTW",
    "NZJHGRCXMYSWBOUFAIVLPEKQDT",
    "FKQHTLXOCBJSPDZRAMEWNIUYGV",
];

pub const NOTCHES: &'static [&'static str; 8] = &[
    "Q", "E", "V", "J", "Z", "ZM", "ZM", "ZM"
];

pub const REFLECTORS: &'static [&'static str; 3] = &[
    "EJMZALYXVBWFCRQUONTSPIKHGD",
    "YRUHQSLDPXNGOKMIEBFZCWVJAT",
    "FVPJIAOYEDRZXWGCTKUQSBNMHL",
];

pub const MAX_PLUGS: usize = 10;
