use std::cmp::Ordering;
use std::collections::HashMap;

fn char_to_num(c: char) -> u32 {
    const RADIX: u32 = 10;
    let d: u32 = c.to_digit(RADIX).unwrap();
    d
}

fn is_valid_pass(num: u32) -> bool {
    //comments solve part one
    let num_str = num.to_string();
    let mut valid = true;
    // let mut repeated = false;
    let mut rep = HashMap::new();

    for (index, i) in num_str.chars().enumerate() {
        let next_digit = num_str.chars().nth(index + 1);

        match next_digit {
            Some(nd) => {
                let c0 = char_to_num(i);
                let c1 = char_to_num(nd);

                match c0.cmp(&c1) {
                    Ordering::Less => {
                        continue;
                    }
                    Ordering::Greater => {
                        valid = false;
                        return valid;
                    }
                    Ordering::Equal => {
                        // repeated = true;
                        let counter = rep.entry(nd).or_insert(1);
                        *counter += 1;
                        continue;
                    }
                }
            }
            None => (),
        }
    }
    println!("{:?}", rep);
    let mut has_two = false;

    for (_, repetitions) in rep {
        if repetitions == 2 {
            has_two = true;
        }
    }

    println!("{:?}", has_two);

    // valid && repeated
    valid && has_two
}

fn main() {
    let mut low = 284639;
    let high = 748759;
    let mut valid = Vec::new();

    while low <= high {
        let is_valid = is_valid_pass(low);
        if is_valid {
            valid.push(low);
        }

        low = low + 1;
    }

    println!("{:?}", valid);
    println!("{:?}", valid.len());
}
