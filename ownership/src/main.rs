fn main() {
    mut_strings();

    // Should break
    // shallow_copy_ish();

    deep_copy_with_clone();

    // With transferring of ownership
    let s1 = String::from("hello");

    let (s1, len) = calculate_length(s1);

    println!("The length of '{}' is {}.\n", s1, len);

    // With references
    let s1: String = String::from("hello, world!");

    let length = calculate_length_with_ref(&s1);

    println!("The length of '{}' is {}.\n", s1, length);

    // // Try to change based on a reference - shouldn't work
    // let s1 = String::from("hello");

    // change(&s1);

    // Mutible change on references
    let mut s1: String = String::from("hello");

    mut_ref_change(&mut s1);

    println!("{}\n", s1);

    // Dangling pointer
    dangle_pointer()
}

fn mut_strings() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}\n", s)
}

// fn shallow_copy_ish() {
//     // Should error
//     let s1 = String::from("hello");
//     let _s2 = s1;

//     println!("{}", s1)
// }

fn deep_copy_with_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}\n", s1, s2);
}


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_with_ref(s: &String) -> usize {
    s.len()
}

// // shouldn't work
// fn change(somestring: &String) {
//     somestring.push_str(", world");
// }

fn mut_ref_change(somestring: &mut String) {
    somestring.push_str(", world");
}

fn dangle_pointer() -> &String {
    // let s = String::from("hello");

    // &s
    // What to do instead
    let s = String::from("hello");

    s
}
