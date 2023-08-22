use std::io;
//use std::num::sqrt;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let input: Vec<f64> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let output: f64 = (input[0] + input[1] + input[2] + input[3]) / 2.0;
    let output: f64 = ((output - input[0]) * (output - input[1]) * (output - input[2]) * (output - input[3])).sqrt();
    println!("{:?}", output);
}
