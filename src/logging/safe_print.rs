use std::io;

pub fn safe_print(string: String) {
    use std::io::Write;

    let mut stdout = io::stdout();

    match stdout.write(string.as_bytes()) {
        Err(error) if error.kind() != io::ErrorKind::BrokenPipe => panic!("An unknown error occurred while printing text!"),
        _                                                       => ()
    }
}

pub fn safe_println(string: String) {
    safe_print(string + "\n");
}
