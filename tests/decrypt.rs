extern crate ultra;

use ultra::decrypt;

macro_rules! decrypt_test {
    ($expected:tt, $ciphertext:tt) => {
        assert_eq!(decrypt($ciphertext).0, $expected);
    }
}


#[test]
fn short_decryption() {
    decrypt_test!(
        "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG",
        "NTZ NTQLZ JMWLL ART BBNOW WZQK KEQ IEVK LZO"
    )
}

#[test]
fn longer_decryption() {
    decrypt_test!(
        "A FEW MILES SOUTH OF SOLEDAD, THE SALINAS RIVER DROPS IN CLOSE TO THE HILLSIDE BANK AND RUNS DEEP AND GREEN. THE WATER IS WARM TOO, FOR IT HAS SLIPPED TWINKLING OVER THE YELLOW SANDS IN THE SUNLIGHT BEFORE REACHING THE NARROW POOL.",
        "Q OCB ODJKE OFZDD BQ YRIRNDW, QZD BOOJHXT PUULE MKCBC FO UAIYG IL ITF ALSUDRGO IKEA LWG SWPA HACL ZDP PDSNQ. WTH OSFWJ TU DKHV VUI, KTM WZ GOR TJDKVAG FXYMZKFWE ENRH BXI PRZZWM GYIGH JL WDM TYSTZLRP EPABZL ZACPUPJY UPU TEQLCX UZBM."
    )
}
