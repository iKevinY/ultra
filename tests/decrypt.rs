extern crate ultra;

use ultra::decrypt;

#[test]
fn basic_decryption() {
    let expected = "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG";
    let ciphertext = "NTZ NTQLZ JMWLL ART BBNOW WZQK KEQ IEVK LZO";
    let (plaintext, rotor, _, _) = decrypt(ciphertext);

    assert_eq!(plaintext, expected);
    assert_eq!(rotor, "142");
}
