use std::io;

pub fn read_stdin() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("error: failed to read user input");
    return buffer.trim().to_string();
}
