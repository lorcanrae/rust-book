#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // Methods
    println!(
        "The area of the rectangle is {} square pixels.\n",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a non-zero width; it is {}\n", rect1.width);
    } else if rect1.width() == false {
        println!("The rectangle has an invalid width!\n")
    }

    // Methods with more Parametrs

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect3));

    println!("");

    // Associated Functions
    let sq = Rectangle::square(40);

    println!("The area of sq is: {}", sq.area());
    println!("Details of sq: {:?}", sq)
}
