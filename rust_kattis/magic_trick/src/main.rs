use std::io;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let first: HashSet<char> = input.trim()
        .chars()
        .collect();
    if input.trim().len() == first.len(){
        println!("1");
    }else{
        println!("0");
    }
}
