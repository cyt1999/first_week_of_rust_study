pub fn print_a_to_z() {
    for c in b'A'..=b'z' {
        print!("{} ", char::from(c));
    }
}
