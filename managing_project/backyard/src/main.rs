use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {
        age: 12,
        water_percent: 98,
    };
    println!("I'm growing {plant:?}!");
}
