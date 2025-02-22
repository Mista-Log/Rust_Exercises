


// Function with Lifetime Annotation
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// Struct with Lifetime
struct TextWrapper<'a> {
    text: &'a str,
}

// Custom Iterator
struct Counter {
    count: u32,
    max: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}


fn main() {
    let s1 = "hello!";
    let s2 = "world!";
    println!("Longest: {}", longest(s1, s2)); // Prints "world!"

    let s = String::from("Hello");
    let wrapper = TextWrapper { text: &s }; // `wrapper.text` is valid as long as `s` exists
    println!("{}", wrapper.text);

    let nums = vec![1,2,3,4]; //Basic Iterator Usage
    let doubled:Vec<_> = nums.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);

    let counter = Counter { count: 0, max: 3 };
    for num in counter {
        println!("{}", num); // Prints 1, 2, 3
    }  
}



