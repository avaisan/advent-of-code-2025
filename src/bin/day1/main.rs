fn main() {
    let input = include_str!("../input/day1.txt");
    let (count, _) = solve_safe_password(input);
    println!("Password is: {}", count);
}

fn solve_safe_password(input: &str) -> (i32, i32) {
    let mut position = 50; // start at 50
    let mut count = 0; // counter for how many times we hit 0

    for line in input.lines() {
        let direction = line.chars().next().unwrap(); // get first char, L or R
        let distance: i32 = line[1..].parse().unwrap(); // get the number after L or R

        // start moving dial
        position = if direction == 'L' {
            (position - distance).rem_euclid(100)
        } else {
            (position + distance) % 100
        };

        if position == 0 {
            count += 1;
        }
    }

    (count, position)
}

#[cfg(test)]
mod tests;
