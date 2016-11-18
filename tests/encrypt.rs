extern crate ultra;

use ultra::enigma::Enigma;

#[test]
fn expected_ciphertext() {
    let mut enigma = Enigma::new("123", "BAT", "HTU", 'B', "");
    assert_eq!(enigma.encrypt("THEQUICKBROWNFOX"), "USSXBXPNRLBSTKQR");
}
