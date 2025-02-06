
// fn main() {
//     println!("Hello World");
// }

use std::io;





// Arithmetic and type casting


fn module() {
    let x= 255_i32;
    let y = 10_i32;

    
    let z= x / y;
    println!("{}", z);

    let a = (i32::MAX as i64) + 1;
    let b = 10_i32;

    let c = a as i32 / b;
    println!("{}", c);



    let a = 5.0 as f32;
    let b = 4.0 as i32;

    let c = a as i32 + b as i32;
    println!("{}", c);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Expected to print a new line");

    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", int_input + 2);


}



fn ownership() {
    let s1 = String::from("hello");
    let s2 = s1;

    let s3 = s2.clone();
    println!("{}", s3);
}


fn main() {


    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);

    let x = 10;
    println!("x is {}", x); 


    {
        let x = 7;
        println!("x is {}", x);
    }

    let a: bool = true;
    println!("{}", a);

    let arr = (2, 3, 7, "f", "h");
    println!("{}", arr.2);

    let mut arr = [2, 3, 5, 6, 9];
    arr[1] = 5;
    println!("{}", arr[1]);


    let  x= x + 4;
    println!("x is {}", x);

    const CONSTANT_VARIABLE: u32 = 50;
    println!("{}", CONSTANT_VARIABLE);



    module();
    ownership();
    
}