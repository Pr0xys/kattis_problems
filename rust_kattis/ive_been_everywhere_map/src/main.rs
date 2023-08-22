use std::io;
use std::collections::HashSet;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let n: i32 = input.trim().parse().unwrap();
    for _ in 0 .. n {
        input.clear();
        io::stdin().read_line(&mut input).expect("error");
        let x: i32 = input.trim().parse().unwrap();
        let mut output: HashSet<String> = HashSet::new();
        for _ in 0 .. x {
            let mut new_input = String::new();
            io::stdin().read_line(&mut new_input).expect("error");
            output.insert(new_input);
        }
        println!("{}",output.len());
    }
}
