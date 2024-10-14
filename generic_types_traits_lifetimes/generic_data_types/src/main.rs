struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T,U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T,U> {

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

}


fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];               // i32 vector
    // let char_list = vec!['y', 'm', 'a', 'q'];                   // char vector
    // let result1 = largest_i32(&number_list);
    // let result2 = largest_char(&char_list);
    // let result3 = largest(&number_list);
    // let result4 = largest(&char_list);
    // println!("The largest number is {result1}");
    // println!("The largest char is {result2}");
    // println!("The largest number is {result3}");
    // println!("The largest char is {result4}");

    let integer = Point2 {
        x: 5,
        y: 10
    };

    let float = Point2 {         
        x: 5.2,
        y: 10.5
    };

    // let mix = Point {        // Don't work cause float != integer
    //     x: 5,
    //     y: 4.0
    // };

    let mix = Point2 {
        x: 5,
        y: 4.5
    };

    println!("X is {} and Y is {}", mix.x(), mix.y());



}

fn _largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn _largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Theses 2 functions could be : 

fn _largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

