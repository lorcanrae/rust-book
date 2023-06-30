fn main() {
    // infinite()

    return_from_loop()
}

fn infinite() {
    loop {
        println!("again!");
    }
}

fn return_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
