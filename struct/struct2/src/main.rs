#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    // //
    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} squara pixels.",
    //     area(width1,height1)
    // );


    // // Tuple
    // let rect1 = (30, 50);

    // println!(
    //     "The area of the rectangle is {} squara pixels.",
    //     area_tuple(rect1)
    // );


    // Struct

    let rect1 = Rect {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} squara pixels.",
        area_struct(&rect1)
    );



}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area_tuple(rect: (u32, u32)) -> u32 {
//     rect.0 * rect.1
// }

fn area_struct(rect: &Rect) -> u32 {
    rect.width * rect.height
}