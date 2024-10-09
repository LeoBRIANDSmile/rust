fn main() {

    // // This part won't be problematic because x has a fixed size
    // let x = 5;
    // let y = x;
    // println!("{x}");
    // println!("{y}");


    // // This part would be problematic if we wan't to print the content in s1
    // let s1 = String::from("Hello");
    // let s2 = s1;
    // println!("{s1}");


    // // This part won't be problematic because we have cloned s1 in s2
    // let s1 = String::from("Hello");
    // let s2 = s1.clone();
    // println!("s1 = {s1}, s2 = {s2}");


    // // Test
    // let s1 = String::from("Hello");
    // let x = 5;

    // let s2 = takes_ownership(s1); // take back ownership in s2

    // dont_take_ownership(x); // don't take ownership

    // println!("{s2}"); // Could print because s2 takes back the ownership of the string "Hello"
    // println!("{x}");
    
    
    // // Transferring ownership of return values
    // let s1 = String::from("Bonjour");
    // let(s2,len) = calculate_length(s1);

    // println!("The len of {s2} is {len}");


    // // Transferring ownership of return values with references
    // let s1 = String::from("Bonjour");
    // let len = calculate_length_with_ref(&s1);

    // println!("The len of {s1} is {len}");


    // // Trying to modify something we borrowed
    // let mut s = String::from("Hello");

    // change_smth_borrowed(&mut s);

    // println!("String is {s}");


    // // Slicing
    // let mut s = String::from("Hello World");
    // let literal_s = "Hello World";
    // // let hello = &s[0..5];   // equals to &s[..5]
    // // let world = &s[6..11];  // equals to &s[6..]

    // let word = first_word(&s);
    
    // println!("First word is {word}");

    // let word = first_word(&literal_s);
    
    // println!("First word is {word}");

    
    // Other slices
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

}

// fn takes_ownership(text: String) -> String {
//     println!("{text}");
//     text // gives ownership
// }

// fn dont_take_ownership(int: i32) {
//     println!("{int}");
// }

// fn calculate_length(text: String) -> (String, usize) {

//     let length = text.len();
//     (text, length)
// }

// fn calculate_length_with_ref(text: &String) -> usize {

//     let length = text.len();
//     length
// }

// fn change_smth_borrowed(s: &mut String) {
//     s.push_str(", world !");
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }

//     &s[..]
// }

