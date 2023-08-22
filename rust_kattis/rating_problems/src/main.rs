use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let input: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut acc: i32 = 0;
    for _ in 0..input[1] {
        let mut new_input = String::new();
        io::stdin().read_line(&mut new_input).expect("error");
        let n: i32 = new_input.trim().parse().unwrap();
        acc += n;
    }
    if input[0] != input[1] {   
        let output_one: f64 = (acc as f64 + (input[0] as f64 - input[1] as f64) * -3.0) / input[0] as f64;
        let output_two: f64 = (acc as f64 + (input[0] as f64 - input[1] as f64) * 3.0) / input[0] as f64;
        println!("{:.2} {:.2}", output_one, output_two);
    } else {
        let output_one: f64 = acc as f64 / input[0] as f64;
        println!("{:.2} {:.2}", output_one, output_one);
    }
}
