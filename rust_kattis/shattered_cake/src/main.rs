use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");
    let big_w: i32 = input
        .trim()
        .parse()
        .unwrap();
    input
        .clear();
    io::stdin()
        .read_line(&mut input)
        .expect("error");
    let n:i32 = input
        .trim()
        .parse()
        .unwrap();
    let mut total_area: i32 = 0;

    for _i in 0 .. n {
        input
            .clear();
        io::stdin()
            .read_line(&mut input)
            .expect("error");
        let input: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x
                 .parse()
                 .unwrap()
                 )
            .collect();
        total_area += input[0] * input[1];
    }

    println!("{}", total_area / big_w);
}
