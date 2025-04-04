fn factorial(num: u32) -> u32 {
    if num == 1 {
        return 1;
    }
    num * factorial(num - 1)
}

fn main() {
    println!("{}", factorial(5));
}
