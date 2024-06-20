mod fingerprint;

use fingerprint::Fingerprint;

fn main() {
    let original = Fingerprint::new("original.txt");
    println!("{}", original);
    original.print_image();
}

// ----- TESTS -----

#[test]
fn fingerprint_equality() {
    let fingerprint1 = Fingerprint::new("original.txt");
    let fingerprint2 = Fingerprint::new("original.txt");
    let fingerprint3 = Fingerprint::new("original.txt");

    assert_eq!(fingerprint1, fingerprint2);
    assert_eq!(fingerprint2, fingerprint1);
    assert_eq!(fingerprint2, fingerprint3);
    assert_eq!(fingerprint1, fingerprint3);
}
