use std::fs::File;

fn main() {
    let _greeting_file = File::open("hello.txt")?;
}

