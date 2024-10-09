use rand::Rng;

// #[derive(Debug)] // so we can inspect the state
// enum UsState {
//     Alabama,
//     Alaska,
//     // ...
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// another example

fn main() {
    //let coin = Coin::Quarter(UsState::Alabama);
    // let value = value_in_cents(coin);
    // println!("The value of the coin is {value} cents");

    let mut dice_roll = roll();
    loop {
        match dice_roll {
            6 => {
                println!("Youhou! It's a 6 !!");
                break;
            }
            _ => {
                println!("It's a {dice_roll}");
                dice_roll = roll()
            }
        }
    }


}

fn roll () -> u8 {
    rand::thread_rng().gen_range(1..=6)
}

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky Penny !");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {state:#?}");
//             25
//         }
//     }
// }