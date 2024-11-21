use string_manipulation::CharString;
use string_manipulation::{str_concat, str_remove, substr, substr_end, substring, substru}; // String and str methods

fn main() {
    // let s1: &str = "test éèçà 123 test";
    // let s2: String = s1.to_owned();

    let s1: &str = "0123456789";
    let s2: String = s1.to_owned();

    println!("substr: {}", substr(s1, 3, 2)); // "34"
    println!("substr: {}", substr(&s2, 3, 2)); // "34"
    println!("substr: {}", s1.substr(3, 2)); // "34"
    println!("substr: {}", s2.substr(3, 2)); // "34"

    println!("substru: {}", substru(s1, 3, 2)); // "34"
    println!("substru: {}", s1.substru(3, 2)); // "34"
    println!("substru: {}", s2.substru(3, 2)); // "34"

    println!("substring: {}", substring(s1, 3, 5)); // "34"
    println!("substring: {}", s1.substring(3, 5)); // "34"

    println!("substr: {}", substr(s1, -2, 2)); // "89", last 2 characters

    println!("substr: {}", substr(s1, -10, 1)); // 0
    println!("substr: {}", substr(s1, 0, 1)); // 0

    println!("substr: {}", substr(s1, 6, isize::MAX)); // "6789"

    println!("substr_end: {}", substr_end(s1, 6)); // "6789", last 2 characters
    println!("substr_end: {}", s1.substr_end(6)); // "6789", last 2 characters

    println!("str_remove: {}", str_remove(s1, 3, 2)); // "01256789"
    println!("str_remove: {}", s1.str_remove(3, 2)); // "01256789"

    match s1.indexof("234", 0) {
        // Result: Some(2)
        Some(pos) => println!("Found at position: {}", pos),
        None => println!("Not found"),
    }

    println!("str_concat: {}", str_concat!(s1, " ", &s2)); // "0123456789 0123456789"
    println!("str_concat: {}", str_concat!(s1, " ", s1)); // "0123456789 0123456789"
}
