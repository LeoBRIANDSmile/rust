use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // Loading values in Hash Map with insert() method
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    // Accessing values in Hash Map with get() method that returns Option<&V>, thus if there is no valuen get() will return None
    let team_name = String::from("Yellow");

    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("The score of {team_name} team is {score}");

    // Iteration over Hash Map
    for (key, value) in &scores {
        println!("{key} : {value}");
    }

    // Hash Map and Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Yellow");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    // Overwriting a value
    scores.insert(String::from("Blue"), 40);

    println!("{scores:?}");

    // Hashing functions provides resistance to DoS attacks involving hash tables

}
