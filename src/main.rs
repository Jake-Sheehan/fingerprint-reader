use std::{
    io::{self, Write},
    process,
};

mod fingerprint;
mod utils;

use fingerprint::Fingerprint;

fn main() {
    let max_try = 3;
    let mut user_try = 0;
    println!("");

    while user_try < max_try {
        print!("Scan your fingerprint: ");
        io::stdout().flush().expect("error: failed to flush stdout");
        let user_input = utils::read_stdin();
        let user_fingerprint = Fingerprint::new(&user_input);
        let original = Fingerprint::new("original.txt");
        println!("");
        user_fingerprint.print_image();
        println!("comparing fingerprints...");

        if user_fingerprint == original {
            println!("success!");
            process::exit(0);
        } else {
            user_try += 1;
        }
    }

    println!("failure!");
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
