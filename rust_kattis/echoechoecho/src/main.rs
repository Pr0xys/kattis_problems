use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("error!");
    let bla = String::from(" ");
    let output = line.trim().to_string() + &bla;
    println!("{}", output.repeat(3));
}
