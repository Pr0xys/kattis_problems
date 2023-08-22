use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let n: i32 = input.trim().parse().unwrap();
    let mut output: i32 = 0;
    for _ in 0..n{
        input.clear();
        io::stdin().read_line(&mut input).expect("error");
        let x: i32 = input.trim().parse().unwrap();
        output += x;
    }

    println!("{}", output - (n-1));
}
