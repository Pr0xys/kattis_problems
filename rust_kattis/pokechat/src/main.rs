use std::io;
//use std::convert::TryInto;

fn main() {
    let mut first_input = String::new();
    io::stdin().read_line(&mut first_input).expect("error");
    
    let mut second_input = String::new();
    io::stdin().read_line(&mut second_input).expect("error");
    
    let numbers: Vec<i32> = second_input.as_bytes()
        .chunks(3)
        .filter_map(|chunk| {
            let num_str = std::str::from_utf8(chunk).unwrap();
            num_str.parse::<i32>().ok()
        })
        .collect();

    let mut output = String::new();

    for i in numbers {
        if i != 0 {
            if let Some(character) = first_input.chars().nth((i - 1).try_into().unwrap()){
                output.push(character);
            }
        }
    }
    
    println!("{}", output);
}
