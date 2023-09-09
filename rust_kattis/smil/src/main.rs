use std::io;

fn main() {
    let smileys: Vec<String> = vec![":)".to_string(),";)".to_string(),":-)".to_string(),";-)".to_string()];
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input error");
    
    
    for lol in &smileys {
        let mut last_pos = 0;
        while let Some(pos) = input[last_pos..].find(lol) {
            let actual_pos = last_pos + pos;
            println!("{}", actual_pos);
            last_pos = actual_pos + 1;
        }
    }
}
