// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6
// }


// fn route(ip_kind: IpAddrKind) {
//     println!("IP kind of {:?}", ip_kind)
// }


// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     route(four);
//     route(six);
// }


// Enum methods

// enum Message {
//     Quit,
//     Move { x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         println!("\ntest method for enum Message")
//     }
// }


// enum Option<T> {
//     None,
//     Some(T),
// }

// main

fn main() {
    // // Basic Enums
    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));

    // // enum methods
    // let m = Message::Write(String::from("hello"));
    // m.call();

    // // Option enums

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;


    // Panic
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y
    // Panic end
}
