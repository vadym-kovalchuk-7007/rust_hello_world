#![allow(unused_variables)]

use std::ops::Range;

const TOUCH_DOWN: u8 = 6;

fn main() {
    let arr = [1, 2, 3];
    let [one, two, three] = arr;
    let mut tpl = ("one", 22);
    tpl.1 = 23;
    let s_range: Range<u8> = 1..32;
    apply_to_jobs(12, "ddd");
    println!("{}", is_even(6));
    println!("{}", is_even(9));
    println!("{:?}", some_text_tup("abra"));
    println!("{:?}", some_text_tup("br"));
}

fn apply_to_jobs(number: i32, title: &str) {
    println!("I'm applying to {number} - {title} jobs");
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
fn some_text_tup(text: &str) -> (bool, bool) {
    let has_a = text.contains("a");
    let has_b = text.contains("b");
    (has_a, has_b)
}
/*
Define an alphabets function that accepts a 'text'
parameter (an &str). The function should return a
tuple of two Booleans. The first Boolean should check
if the text contains the letter 'a'. The second
Boolean should check if the text contains the letter
'z'. You can use the 'contains' method to check if a
string contains a specific character. See the documentation:
https://doc.rust-lang.org/std/primitive.str.html#method.contains

Examples:
println!("{:?}", alphabets("aardvark")); -> (true, false)
println!("{:?}", alphabets("zoology"));  -> (false, true)
println!("{:?}", alphabets("zebra"));    -> (true, true)
*/
