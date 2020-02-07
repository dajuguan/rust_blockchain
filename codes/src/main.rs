use rand::Rng;
use std::cmp::Ordering;
use std::io;
const max: u32 = 100;

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn main() {
    let mystring = String::from("hello rust");
    let word = first_word(&mystring[..]);
    println!("{}", word);
    let mystr = "hello rust";
    let word_str = first_word(&mystr);
    println!("{}", word_str);
}
