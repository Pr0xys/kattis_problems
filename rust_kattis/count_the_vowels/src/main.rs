use std::io;

fn main() {
    let vowel_list: Vec<char>  = vec!['a', 'e', 'i', 'o', 'u'];
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");
    let mut acc: i32 = 0;
    let input = input.trim().to_lowercase();
    for ch in input.chars(){
        for i in &vowel_list{
            if ch  == *i {
                acc += 1;
            }
        }
    }
    println!("{}", acc);
}
