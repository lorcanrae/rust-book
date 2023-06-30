fn main() {
    println!("One");
    one();

    println!("\nTwo");
    two();

    println!("\nThree");
    three();

    println!("\nFour");
    four();

    // Shouldn't work because rust is strongly typed
    // println!("\nFive");
    // five();
}

fn one() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn two() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}

fn three() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn four() {
    let condition = true;
    let number = if condition {5} else {6};

    println!("The value of number is: {number}");
}

// fn five() {
//     // this should break
//     let condition = true;

//     let number = if condition { 5 } else { "six" };

//     println!("The value of number is: {number}");
// }
