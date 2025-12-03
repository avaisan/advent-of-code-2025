/* Problem framing:
"Sequence of digits repeated twice" is the key word. Our logic in code can be that we split given number in half and compare halves. If halves are the same, it is an invalid ID. If it is an odd number, it cannot be invalid.
-> Iterate through each range, check each number for invalid, gather invalids in an array, them sum up the array to get the answer.
*/

fn main() {
    let input = include_str!("../input/day2.txt");
    let invalid_ids = invalid_ids(input);
    println!("Invalid IDs: {:?}", invalid_ids);
    println!("Sum of invalid IDs: {}", invalid_ids.iter().sum::<i64>());
}

fn invalid_ids(input: &str) -> Vec<i64> {
    let mut invalid_ids = Vec::new();
    
    // Split ranges by commas using for loop, then split ranges by dash.
    for range in input.split(',') {
        let parts: Vec<&str> = range.split('-').collect(); // collect range into a vector parts
        let start: i64 = parts[0].parse().unwrap();
        let end: i64 = parts[1].parse().unwrap();
        
        // Now we check each number for validity and push invalids to array
        for num in start..=end {
            if is_invalid(num) {
                invalid_ids.push(num);
            }
        }
    }
    
    invalid_ids
}

fn is_invalid(num: i64) -> bool {
    let id = num.to_string();
    
    // Easy win: odd numbers cannot repeat, so these are valid
    if id.len() % 2 != 0 {
        return false;
    }
    
    // For even numbers, split and compare halves
    let split = id.len() / 2;
    &id[0..split] == &id[split..]
}

#[cfg(test)]
mod tests;
