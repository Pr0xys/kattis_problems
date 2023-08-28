use std::io;

fn main() {
    let mut input = String::new();
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
    
    let a_1: i32 = input[1] * input[2];
    let a_2: i32 = input[1] * (input[0] - input[2]);
    let a_3: i32 = (input[0] - input[1]) * input[2];
    let a_4: i32 = (input[0] - input[1]) * (input[0] - input[2]);

    let mut output_list: Vec<i32> = Vec::new();
    output_list.push(a_1);
    output_list.push(a_2);
    output_list.push(a_3);
    output_list.push(a_4);

    output_list.sort();
    output_list.reverse();
    
    let  output: i32 = output_list[0] * 4;
    println!("{}", output);
}
