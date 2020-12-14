pub fn print_str(e: [&str; 3]) {
    for element in e.iter() {
        print!("{}\n", element)
    }
}
