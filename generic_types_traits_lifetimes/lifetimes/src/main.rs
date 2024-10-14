fn main() {
    let s: &'static str = "I have a static lifetime.";
    let string1 = String::from("abcd");
    let string2 = "uvwxyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // // Examples of lifetime annotation syntax

    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {           // named lifetime parameter
    if s1.len() > s2.len() {
        s1
    }
    else {
        s2
    }
    
}