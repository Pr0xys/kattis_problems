use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let p: f64 = input.trim().parse().unwrap();

    let p = p / 100.0;

    let ratio_option_one = 1.0 / p;
    let ratio_option_two = 1.0 / (1.0 - p);

    println!("{:.10}", ratio_option_one);
    println!("{:.10}", ratio_option_two);
}
