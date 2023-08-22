use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let n: i32 = input.trim().parse().unwrap();

    for _ in 0..n{
        input.clear();
        io::stdin().read_line(&mut input).expect("error");
        let input: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let output: i32 = input[1] * (input[1] +1) / 2 + input[1];
        println!("{} {}", input[0], output);
    }
}
