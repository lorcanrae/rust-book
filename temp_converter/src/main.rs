use std::io;

fn main() {
    user_input()

    // let temp = converter(100.);

    // println!("temperature: {temp:.2}");
}

fn user_input() {
    println!("Fahrenheit to Celsius converter");

    loop {
        println!("Input a temperature in Fahrenheit!");

        let mut temp: String = String::new();
        // let mut temp = 0;

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let mut temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        temp = (temp - 32.0) * (5.0 / 9.0);

        println!("The temperature in Celsius is: {temp:.2}");
    }
}

fn converter(f_temp: f64) -> f64 {
    let c_temp: f64 = (f_temp - 32.) * (5. / 9.);
    c_temp
}
