fn main() {
    let input = include_str!("../input/day1.txt");
    let (count_p1, _) = solve_safe_password_p1(input);
    println!("Part 1 Password: {}", count_p1);
    
    let count_p2 = solve_safe_password_p2(input);
    println!("Part 2 Password: {}", count_p2);
}

fn solve_safe_password_p1(input: &str) -> (i32, i32) {
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

// part 2: now we need to count all times we hit 0.
// Move dial one step at a time, count every time we hit 0 and add it to counter. That should be our answer.
fn solve_safe_password_p2(input: &str) -> i32 {
    let mut position: i32 = 50;
    let mut count = 0;

    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].parse().unwrap();

        // Move dial one click at a time
        for _ in 0..distance {
            position = if direction == 'L' {
                (position - 1).rem_euclid(100)
            } else {
                (position + 1) % 100
            };
            
            if position == 0 {
                count += 1;
            }
        }
    }

    count
}


#[cfg(test)]
mod tests;
