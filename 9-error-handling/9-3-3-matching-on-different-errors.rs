use std::fs::File;

fn main() {
    // let _greeting_file = File::open("hello.txt").unwrap();
    let _greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
}

