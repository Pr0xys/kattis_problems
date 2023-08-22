use std::io;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");
    let input_list = input
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>();
    let input_set = input
        .trim()
        .split_whitespace()
        .collect::<HashSet<&str>>();
    
    if input_list.len() == input_set.len(){
        println!("yes");
    } else {
        println!("no");
    }
}
