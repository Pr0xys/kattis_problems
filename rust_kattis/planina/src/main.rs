use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let input:u32 = input.trim().parse().unwrap();
    let output: u32 = (2_u32.pow(input)+1).pow(2);
    println!("{}", output);
}
