fn main() {
    let mut s = String::new();
    s += "inserted content";
    println!("{s}");
    let data = "initial contents";
    let s = data.to_string();
    println!("{s}");
    let s = "initial contents".to_string();
    println!("{s}");
    let s = String::from("initial contents");
    println!("{s}");
}
