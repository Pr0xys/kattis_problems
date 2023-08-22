use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    input.clear();
    io::stdin().read_line(&mut input).expect("error");
    let input: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut counter = 0;
    for i in input {
        if i < 0 {
            counter += 1;
        }
    }
    println!("{}", counter);
}
