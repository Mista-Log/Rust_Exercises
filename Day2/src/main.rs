


use std::io;


fn test() {

    println!("Lets get it done");

}

fn add_numbers(x: i32, y:i32) {
    println!("The sum of the input is: {}", x + y);
}


fn control_flow() {
    let number = 3;
    if number < 5 {
        println!("The number is less than 5");
    } else {
        println!("The number is greater than 5");
    }
}


fn borrowing() {
    let x = 5;
    let y = &x;
    let z = &x;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}


fn subtract() {
    let a = 10;
    let b = 5;
    let result = a - b;
    println!("The result of subtraction is: {}", result);
}



fn immutable_function(x: &String) {
    println!("The amount of Letters are: {}", x.len());
}


fn to_uppercase(s: &mut String) {
    s.make_ascii_uppercase();
}


fn main() {
    println!("Hello, world!");
    let mut input = String::new();


    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);

    add_numbers(6, 8);

    // <    Less than operator
    // >    Grater than operator
    // <=   Gess than or greater than
    // >=   Greater than or less than
    // !=   Not equal to
    // ==   Is equal to
    // These are the conditional signs



    let cond = (2 as f32) <= 2.2;
    println!("{}", cond);
    test();
    subtract();



    let mut my_string = String::from("Hello rust");
    immutable_function(&my_string);
    immutable_function(&my_string);


    to_uppercase(&mut my_string);
    println!("{}", my_string);
    // && Syntax for and operator


    control_flow();

    let mut a = 5;
    while a < 10 {
        println!("The value of a is: {}", a);
        a += 1;
    }


    borrowing();
}




