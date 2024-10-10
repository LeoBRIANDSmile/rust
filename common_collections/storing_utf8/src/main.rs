fn main() {
    let mut s = String::new();
    let s2 = " string";
    let s3 = String::from(" ! :)");

    s.push_str("New");
    s.push_str(s2);

    s = s + &s3;
    
    println!("{s}");

    // Methods for iterating Over Strings

    let string = String::from("Hello world");

    for c in string.chars() {
        println!("{c}");
    }

    for c in string.bytes() {
        println!("{c}");
    }

}
