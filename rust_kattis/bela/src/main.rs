use std::io;

fn main() {

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("error");

    let input_list: Vec<&str> = input
        .trim()
        .split_whitespace()
        .collect();

    let n: usize = input_list[0]
        .parse()
        .unwrap();

    let trydis = input_list[1]
        .chars()
        .next()
        .unwrap();

    let number = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7'];
    let dominant = [11, 4, 3, 20, 10, 14, 0, 0];
    let non_dominant = [11, 4, 3, 2, 10, 0, 0, 0];

    let mut output = 0;

    for _ in 0..(n * 4) {
        input
            .clear();

        io::stdin()
            .read_line(&mut input)
            .expect("error2");

        let s: Vec<char> = input
            .trim()
            .chars()
            .collect();

        let index = number
            .iter()
            .position(|&x| x == s[0])
            .unwrap();
        
        if s[1] == trydis {
            output += dominant[index];
        } else {
            output += non_dominant[index];
        }
    }

    println!("{}", output);
}
