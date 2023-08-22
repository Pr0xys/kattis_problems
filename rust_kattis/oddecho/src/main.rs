use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error!");
    let n: i32 = input.trim().parse().unwrap();

    for i in 1..n + 1 {
        input.clear();
        io::stdin().read_line(&mut input).expect("error!");
        if i % 2 != 0 {
            println!("{}", input.trim());
        }
    }
}
