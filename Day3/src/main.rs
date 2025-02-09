struct User{
    name: String,
    age: u32,
}



enum  Variants {
    Up,
    Down,
    Right,
    Left,

}

fn move_player(variation: Variants) {
    match variation {
        Variants::Up => println!("Moving Up"),
        Variants::Down => println!("Moving Down"),
        Variants::Right => println!("Moving Right"),
        Variants::Left => println!("Moving Left"),
    }
}




enum Message {
    Text(String),
    Move { x: i32, y: i32 },
    Quit,
}

fn process_message(msg: Message) {
    match msg {
        Message::Text(content) => println!("Message: {}", content),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Quit => println!("Goodbye!"),
    }
}





fn main() {
    let user = User{
        name: String::from("Alice"),
        age: 39,

    };
    println!("My name is {}, and i am {} years old", user.name, user.age);
    println!("Hello, world!");

    let movement = Variants::Down;
    move_player(movement);
    let movement1 = Variants::Up;
    move_player(movement1);
    let movement2 = Variants::Left;
    move_player(movement2);
    let movement3 = Variants::Right;
    move_player(movement3);

    let number = 23;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3..=5 => println!("Between three and five"),
        _=> println!("Something else"),

    }


    let msg = Message::Move { x: 10, y: 20 };
    process_message(msg);







}
