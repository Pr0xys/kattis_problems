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
    
    let mut output: Vec<i32> = Vec::new();

    for _ in 0 .. n{
        input
            .clear();
        io::stdin()
            .read_line(&mut input)
            .expect("error");
        output.push(input
                    .trim()
                    .parse::<i32>()
                    .unwrap());
    }

    output.reverse();

    for i in &output {
        println!("{}", i);
    }
}
