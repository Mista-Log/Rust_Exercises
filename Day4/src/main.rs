

use std::collections::HashMap;
use std::io::Error;
use std::fs::File;




fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a/b)
    }
}



fn open_file(filename: &str) -> Result<File, Error> {
    File::open(filename)
}




fn main() {
    // println!("Hello, world!");


    //Vector(Vec<T>)
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    numbers.push(6); // Add an element
    println!("{:?}", numbers); // Output: [1, 2, 3, 4, 5, 6]



    let number = vec![1,2,3,4];
    let first = number[0];
    println!("The first number is {}", first);

    match number.get(1) {
        Some(value) => println!("Second element: {}", value),
        None => println!("No value found"),
    }

    for num in &number {
        println!("{}", num);
    }

    for num in &mut numbers {
        *num += 10;
    }

    println!("{:?}", numbers);


    //Strings (String & str)
    let mut s = String::from("Hello");
    s.push_str(", world");
    println!("{}", s);


    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = s1 + " " + &s2;
    println!("{}", s3);


    let s1 = String::from("Hello");
    let s2 = String::from("Rust");


    let s3 = format!("{} {}", s1, s2);
    println!("{}", s3);


    //HashMaps(HashMap<K, V>)
    let mut scores = HashMap::new();

    scores.insert(String::from("Alice"), 90);
    scores.insert(String::from("ABob"), 85);


    println!("{:?}", scores);





    //Error Handling in rust
    match divide(10, 2) {
        Some(result) => println!("Result: {}", result),
        None => println!("Cannot divide by zero!"),
    }


    match open_file("Cargo.lock") {
        Ok(file) => println!("File opened successfully! {:?}", file),
        Err(error) => println!("Error opening file: {}", error),
    }




}
