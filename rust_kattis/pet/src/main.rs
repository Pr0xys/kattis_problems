use std::io;
use std::collections::HashMap;

fn main() {
    let mut output_map: HashMap<i32,i32>  = HashMap::new();    
    for i in 1 .. 6 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error");
        let input: i32 = input.trim()
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .sum();
        output_map.insert(i,input);
    }
    let (max_key, max_value) = output_map.iter()
        .max_by_key(|(_, &value)| value)
        .map(|(&key, &value)| (key, value))
        .unwrap_or((0, 0));
    /*
     * if the HashMap is not empty, max_by_key returns Some((&key, &value)), where &key and &value are references to the key and value of the pair with the highest value.
     * The map method applies the closure to this Some, resulting in Some((key, value)), where key and value are the actual key and value (not references).
     * This Some((key, value)) is then unwrapped with unwrap_or((0, 0)), resulting in the tuple (key, value) if it was Some, or (0, 0) if it was None.
     * This tuple is then stored in (max_key, max_value).
     */

     println!("{} {}", max_key, max_value);
}
