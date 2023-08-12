#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // ##### Using regular variables #####

    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} pixels.",
    //     area(width1, height1)
    // );

    // let area = area(width1, height1);
    // println!("width is: {}", width1);


    // ##### Using tuples #####

    // let rect1 = (30, 50);

    // println!(
    //     "The area of the rectangle is {} square pixels,",
    //     area_tuple(rect1)
    // );

    // ##### Now with structs #####

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "\nThe area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    println!("rect1 is {:#?}\n", rect1);


    // ##### Using dbg! #####
    // neat, I like it.

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
