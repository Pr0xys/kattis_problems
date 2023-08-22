use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let mut input: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    input.sort();
    if input[0] == input[1] {
        println!("{}", input[0]+1);
    } else {
        for i in input[0]+1..input[1]+2{
            println!("{}", i);
        }
    }
}
