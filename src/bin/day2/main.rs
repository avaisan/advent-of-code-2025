fn main() {
    let input = include_str!("../input/day2.txt");
    
    let invalid_ids_p1 = invalid_ids(input, is_invalid_p1);
    println!("Part 1: Invalid IDs: {:?}", invalid_ids_p1);
    println!("Part 1: Sum: {}", invalid_ids_p1.iter().sum::<i64>());
    
    let invalid_ids_p2 = invalid_ids(input, is_invalid_p2);
    println!("Part 2: Invalid IDs: {:?}", invalid_ids_p2);
    println!("Part 2: Sum: {}", invalid_ids_p2.iter().sum::<i64>());
}

fn invalid_ids(input: &str, is_invalid: fn(i64) -> bool) -> Vec<i64> {
    let mut invalid_ids = Vec::new();
    
    for range in input.split(',') {
        let parts: Vec<&str> = range.split('-').collect();
        let start: i64 = parts[0].parse().unwrap();
        let end: i64 = parts[1].parse().unwrap();
        
        for id in start..=end {
            if is_invalid(id) {
                invalid_ids.push(id);
            }
        }
    }
    
    invalid_ids
}

/* Part 1:
"Sequence of digits repeated twice" is the key word. Our logic in code can be that we split given number in half and compare halves. If halves are the same, it is an invalid ID. If it is an odd number, it cannot be invalid.
-> Iterate through each range, check each number for invalid, gather invalids in an array, them sum up the array to get the answer.
*/
fn is_invalid_p1(num: i64) -> bool {
    let id = num.to_string();
    
    // Easy win: odd numbers cannot repeat, so these are valid
    if id.len() % 2 != 0 {
        return false;
    }
    
    // For even numbers, split and compare halves
    let split = id.len() / 2;
    &id[0..split] == &id[split..]
}

/*
Part 2: 
"ID is invalid if it is made only of some sequence of digits repeated". So we need to check for repeating patterns, previous logic of splitting into halves, or amending logic by also checking for odd numbers, won't work.
I will go for brute force solution. For each pattern length 1,2,3,4,..., check if by going halfway and repeating it, we get the same number. 
E.g. 1111 -> pattern length 1, repeat 1 by pattern length 4 -> we should get original number 1111. If yes, invalid.
1245 -> pattern length 1, repeat 1 by pattern length 4 -> we get 1111, which is not 1245 -> valid. For loop pattern length 2, get 12, repeat by pattern length 2 -> 1212, still not 1245. If for loop passes, the ID was valid.
*/
fn is_invalid_p2(num: i64) -> bool {
    let id = num.to_string();
    let length = id.len();
    
    for pattern_length in 1..=length/2 {
        let pattern = &id[0..pattern_length];
        
        // Repeat pattern and check if it matches original
        let repeated = pattern.repeat(length / pattern_length);
        if repeated == id {
            return true;
        }
    }
    
    false
}


#[cfg(test)]
mod tests;
