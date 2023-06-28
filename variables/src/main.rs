fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");


    let mut y = 5;
    println!("The value of y is: {y}");

    y = 10;
    println!("The value of y is: {y}");
}
