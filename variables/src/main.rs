fn main() {
    // let x = 5;

    // // Overshadowing previous instance of x
    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");


    // // The following works because y is the same data type
    // // The reassignment wouldn't work if y was reassigned to a different type.
    // let mut y = 5;
    // println!("The value of y is: {y}");

    // y = 10;
    // println!("The value of y is: {y}");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
