
//Generic function
fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
    //Generic Sructs
    struct Point<T> {
        x: T,
        y: T,
    }
    // //Generic Enums

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }


// Traits in Rust

trait Speak {
    fn speak(&self);
}
struct Dog;
struct Cat;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}



fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}



fn main() {
    println!("Hello, world!");

    println!("The largest number is {}", largest(1, 2));
    let int_point = Point { x: 3, y: 5 };
    println!("The point is x: {}, y: {}", int_point.x, int_point.y);

    match divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }



    let dog = Dog;
    let cat = Cat;

    dog.speak(); // Output: Woof!
    cat.speak(); // Output: Meow!


}
