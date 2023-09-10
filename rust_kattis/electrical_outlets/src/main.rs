use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input error");
    let n: i32 = input.trim().parse().unwrap();
    

    for _ in 0 .. n {
        input
            .clear();
        io::stdin()
            .read_line(&mut input)
            .expect("input error");

        let output = input
            .split_whitespace()
            .enumerate()
            .filter(|(i, _)| *i != 0)
            .map(|(_, x)| x.parse::<i32>().unwrap())
            .map(|x| x-1)
            .fold(0, |acc, x| acc + x);
        println!("{}", output + 1);
    }
}
