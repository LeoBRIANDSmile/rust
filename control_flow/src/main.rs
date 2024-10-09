fn main() {

    // // Condition

    // let condition = false;
    // let number = if condition {3} else {8};


    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

//////////////////////////////////////////////////////

    // // Loop

    // loop{
    //     println!("infinite loop");
    // }

//////////////////////////////////////////////////////

    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;

    //     println!("The counter is {counter}");

    //     if counter == 10 {
    //         break counter*2;
    //     }
    // };
    
    // println!("The result is {result}");


//////////////////////////////////////////////////////

    // // While
    // let mut number = 3;

    // while number >=0 {
    //     println!("{number}");

    //     number-=1;
    // }

    // println!("IT'S OVER !!!");

//////////////////////////////////////////////////////

    // For

    let a: [i32;5] = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println!("the value {index} is : {}", a[index]);

        index+=1;
    }
    // Could be replaced by :

    for number in (0..5) {
        println!("the value {number} is : {}", a[number]);
    }
    // Could be replaced by : 

    for element in a {
        println!("the value is {element}")
    }
}
