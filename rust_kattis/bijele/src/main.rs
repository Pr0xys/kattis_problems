use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let input: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let standard_set: Vec<i32> = vec![1, 1, 2, 2, 2, 8];
     for i in 0..6 {
        println!("{}", standard_set[i] - input[i]);
    }
}
