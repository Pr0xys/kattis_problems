use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let n: i32 = input.trim().parse().unwrap();

    for _ in 0 .. n{
        input.clear();
        io::stdin().read_line(&mut input).expect("error");
        let x: i32 = input.trim().parse().unwrap();
        if x % 2 == 0 { 
            println!("{} is even", x);
        } else {
            println!("{} is odd", x);
        }
    }
}
