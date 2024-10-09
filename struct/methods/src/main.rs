#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rectangle: &Rect) -> bool{
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {

    // // Use methods
    // let rect1 = Rect {
    //     width: 30,
    //     height: 50,
    // };

    // println!("rect1 is {:#?}", rect1);

    // println!(
    //     "The area of the rectangle is {} squara pixels.",
    //     rect1.area()
    // );
    
    
    // // Methodes with more parameters
    // let rect1 = Rect {
    //     width: 30,
    //     height: 50,
    // };

    // let rect2 = Rect {
    //     width: 10,
    //     height: 40,
    // };

    // let rect3 = Rect {
    //     width: 60,
    //     height: 45,
    // };

    // println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


    // Create a square

    let square1 = Rect::square(50);

    println!("{:#?}",square1 );

}