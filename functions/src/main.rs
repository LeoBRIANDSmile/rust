fn main() {
    let a = 5;
    let b = 6;

    let result = addition(a,b);

    println!("Result of {a} + {b} is {result}");
}

fn addition(a: i32, b: i32) -> i32 {
    a+b
}