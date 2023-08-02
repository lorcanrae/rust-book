fn main() {
    let s = String::from("hello cargo");

    let word = first_word(&s);

    // s.clear();

    println!("The first word of {} is {}\n", s, word);


    let string_literal = "hello world";

    let word = first_word(string_literal);
    println!("{}\n", word);

    let word = first_word(&string_literal);
    println!("{}\n", word);

    let word = first_word(&string_literal[..6]);
    println!("{}\n", word);

    // println!("word is {}", s);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
