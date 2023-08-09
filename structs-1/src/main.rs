struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("lradical"),
        email: String::from("lrad@awesomeville.rad"),
        sign_in_count: 1,
    };


    println!("Orginal email is: {}\n", user1.email);

    user1.email = String::from("different@email.forscience");

    println!("The updated email is: {}\n", user1.email);

    // verbose way of copying from another struct
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}\n", user2.email);

    // errors out because user1.username was move into user2
    // println!("{}\n\n", user1.username);

    println!("TUPLE STRUCTS\n");

    let black = Color(100, 200, 300);
    let origin = Point(0, 0, 0);
    let red = black.0;

    println!("{}", red);
    println!("{}", black.0);
}


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // username: username, // replaced with shorthand
        username,
        // email: email, // replaced with shorthand
        email,
        sign_in_count: 1,
    }
}
