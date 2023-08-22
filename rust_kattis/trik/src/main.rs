use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut output: i32 = 1;
    let input = input.trim();
    for i in input.chars() {
        match i {
            'A' => {
                if output == 1 {
                    output = 2;
                    continue;
                }
                if output == 2{
                    output = 1;
                    continue;
                }
            },
            'B' => {
                if output == 2 {
                    output = 3;
                    continue;
                }
                if output == 3{
                    output = 2;
                    continue;
                }
            },
            'C' => {
                if output == 1 {
                    output = 3;
                    continue;
                }
                if output == 3 {
                    output = 1;
                    continue;
                }
            },
            _ => println!("Unhandled character"),
        }
    }

    println!("{}", output);
}

