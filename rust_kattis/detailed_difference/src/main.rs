use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");

    let n:i32 = input
        .trim()
        .parse()
        .unwrap();

    (0..n).for_each(|_| {
        let mut first_string = String::new();
        io::stdin()
            .read_line(&mut first_string)
            .expect("error");
        let mut second_string = String::new();
        io::stdin()
            .read_line(&mut second_string)
            .expect("error");

        let result: String = first_string
            .trim()
            .chars()
            .zip(second_string
                 .trim()
                 .chars())
            .map(|(x, y)| match x == y {
                true => '.',
                false => '*',
                })
            .collect();
        
        println!("{}", first_string.trim());
        println!("{}", second_string.trim());
        println!("{}", result);
        print!("\n");
        print!("\n");
    });
}
