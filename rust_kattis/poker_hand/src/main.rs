use std::io;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let input = input.trim().replace(&['C','D','H','S'][..], "");
    let input: String = input.split_whitespace().collect();
    let mut char_map: HashMap<char,i32> = HashMap::new();

    for chars in input.chars(){
        if char_map.contains_key(&chars) {
            let n:i32 = *char_map.get(&chars).unwrap_or(&0) + 1;
            char_map.insert(chars, n);
        } else {
            char_map.insert(chars, 1);
        }
    }
    let output: i32 = *char_map.values().max().unwrap_or(&0);
    println!("{}", output);
}
