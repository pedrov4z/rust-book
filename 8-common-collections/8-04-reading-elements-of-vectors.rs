fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // Indexing
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // Get method
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element.")
    }

    // let does_not_exist = &v[100];
    // The line above would cause a panic
    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(value) => println!("At index 100 we have: {value}"),
        None => println!("There is nothing at vector index 100")
    }

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6);
    // The line above would not compile because there is an immutable borrow of the first vector element

    println!("The first element is: {first}");
}
