#![allow(unused_variables)]

mod references;

use std::ops::Range;

fn count_down(num: u8) {
    if num == 0 {
        println!("Is zero");
    } else {
        println!("{}", num);
        count_down(num - 1);
    }
}

fn factorial(num: u32) -> u32 {
    if num == 1 {
        return 1;
    }
    num * factorial(num - 1)
}

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
    count_down(5);
    println!("{}", color_to_number("green"));
    println!("{}", factorial(5));
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

fn color_to_number(color: &str) -> u8 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}
