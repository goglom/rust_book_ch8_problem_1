use std::collections::HashMap;
use std::io::{self, Write};

fn get_median(vec: &mut Vec<i32>) -> f64 {
    let length = vec.len();
    let half_length = length / 2;
    let mid1: i32 = *vec.select_nth_unstable(half_length).1;

    let median_val: f64 = match length % 2 {
        0 => {
            let mid2: i32 = *vec.select_nth_unstable(half_length - 1).1;
            (mid1 + mid2) as f64 / 2.0
        }
        _ => mid1 as f64,
    };

    median_val
}

fn get_mode(vec: &Vec<i32>) -> i32 {
    let mut freqs: HashMap<i32, usize> = HashMap::new();

    for &num in vec {
        let count = freqs.entry(num).or_insert(0);
        *count += 1;
    }

    *freqs
        .iter()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap()
        .0
}

fn main() {
    let mut nums: Vec<i32> = Vec::new();

    'outer: loop {
        let mut input = String::new();

        if let Err(error) = io::stdin().read_line(&mut input) {
            println!("Error occured while reading input: {error}. Try again...");
        }

        if input.is_empty() {
            continue;
        }
        
        for c in input.split_whitespace() {
            match c.parse::<i32>() {
                Ok(num) => {
                    nums.push(num);
                }
                Err(error) => {
                    print!(
                        "Expected only numbers in input. Parse error: '{error}' in string: '{c}'\n"
                    );
                    io::stdout().flush().unwrap();
                    nums.clear();
                    continue 'outer;
                }
            }
        }
        break;
    }
    let median_val = get_median(&mut nums);

    println!("Median value of input array is: {median_val}");

    let mode_val = get_mode(&nums);

    println!("Mode of input array is: {mode_val}");
}
