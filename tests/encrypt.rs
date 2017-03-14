extern crate ultra;

use ultra::Enigma;

#[test]
fn expected_ciphertext() {
    let mut enigma = Enigma::new("123", "BAT", "HTU", 'B', "");
    assert_eq!(enigma.encrypt("THEQUICKBROWNFOX"), "USSXBXPNRLBSTKQR");
}

#[test]
fn turnover_points() {
    let mut enigma = Enigma::new("123", "AAA", "ADU", 'B', "");
    assert_eq!(enigma.encrypt("THEQUICKBROWNFOX"), "ACGXKHKYCBVQZMJM");
}
