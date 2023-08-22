use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let x: i32 = input.trim().parse().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).expect("error");
    let i: i32 = input.trim().parse().unwrap();
    let mut output: i32 = 0;

    for _ in 0..i{
        input.clear();
        io::stdin().read_line(&mut input).expect("error");
        let numb: i32 = input.trim().parse().unwrap();
        output += x - numb;
    }
    output += x; 

    println!("{}", output);
}
