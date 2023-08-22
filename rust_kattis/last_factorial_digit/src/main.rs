use std::io;

fn factorial(n: u64) -> u64 {
    match n {
        0 => {1},
        _ => {n * factorial(n-1)},
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");
    let n: i32 = input
        .trim()
        .parse()
        .unwrap();

    for _ in 0 .. n{
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("error22");
        let x: u64 = input
            .trim()
            .parse()
            .unwrap();
        
        let output: u64 = factorial(x);
        println!("{}", output % 10);
    }
}
