use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");

    match input.to_lowercase().contains("cov") {
        true => println!("Veikur!"),
        false => println!("Ekki veikur!"),
    }
}
