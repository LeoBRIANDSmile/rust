// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     addr: String,
// }

// is equal to 

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// or

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// antoher example

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method
    }
}
fn main() {
    let home_ip_addr = IpAddr::V4(127, 0, 0, 1);
    let m = Message::Write(String::from("hello"));
    m.call();
}
