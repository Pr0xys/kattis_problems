use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error!");
    let number:i32 = input.trim().parse().unwrap();
    let mut counter:i32 = number;

    for i in 0..number {
        match i % 2 {
            0 => {counter -= 1; if counter == 0{ println!("Alice");}},
            1 => {counter -= 1; if counter == 0{ println!("Bob");}},
            _ => println!("damn")
        }
    }
}

