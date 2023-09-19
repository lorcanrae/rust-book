// A more concise version of `enums_structs.rs`
// The name of each variant in the enum becomes a function that
// constructs an instance of the enum.


// A Basic Version

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(String::from("127.0.0.1"));

//     let loopback = IpAddr::V6(String::from("::1"));
// }


// Controlling different data types in each enum

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Some more enuums

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("test message, contains: {}", &self)
    }
}


// main

fn main() {
    // let home = IpAddr::V4(127, 0, 0, 1);

    // let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"))

    m.call()
}
