use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Not a number!");
    let input: i32 = input.trim().parse().unwrap();

    for i in 1..input+1 {
        println!("{} Abracadabra", i);
    }
}
