use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let list_numbers: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    println!("{}", list_numbers[0] + list_numbers[1]);
}
