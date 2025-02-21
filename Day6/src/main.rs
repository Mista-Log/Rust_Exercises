


// Function with Lifetime Annotation
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// Struct with Lifetime
struct TextWrapper<'a> {
    text: &'a str,
}

fn main() {
    let s1 = "hello!";
    let s2 = "world!";
    println!("Longest: {}", longest(s1, s2)); // Prints "world!"

    let s = String::from("Hello");
    let wrapper = TextWrapper { text: &s }; // `wrapper.text` is valid as long as `s` exists
    println!("{}", wrapper.text);
}


