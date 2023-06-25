fn main() {
    use std::collections::HashMap;
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{field_name}: {field_value}");
    // the line above is invalid, the values are owned by the map at this point 
}
