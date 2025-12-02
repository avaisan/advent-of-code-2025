use super::*;

/*
For example, suppose the attached document contained the following rotations:

L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
Following these rotations would cause the dial to move as follows:

The dial starts by pointing at 50.
The dial is rotated L68 to point at 82.
The dial is rotated L30 to point at 52.
The dial is rotated R48 to point at 0.
The dial is rotated L5 to point at 95.
The dial is rotated R60 to point at 55.
The dial is rotated L55 to point at 0.
The dial is rotated L1 to point at 99.
The dial is rotated L99 to point at 0.
The dial is rotated R14 to point at 14.
The dial is rotated L82 to point at 32.
Because the dial points at 0 a total of three times during this process, the password in this example is 3.
*/

// Start from 50, go R50 -> dial hits 0 once, dial should be at 0
#[test]
fn test_one_right_rotation() {
    let (count, position) = solve_safe_password("R50");
    assert_eq!(count, 1);
    assert_eq!(position, 0);
}

// Start from 50, go R1 then L1 -> never hits 0, dial should be back at 50
#[test]
fn test_right_then_left() {
    let (count, position) = solve_safe_password("R1\nL1");
    assert_eq!(count, 0);
    assert_eq!(position, 50);
}

// Start from 50, go R50 twice -> hits 0 once, dial should be back at 50
#[test]
fn test_right_twice() {
    let (count, position) = solve_safe_password("R50\nR50");
    assert_eq!(count, 1);
    assert_eq!(position, 50);
}


// Start from 50, go L51 -> never hits 0, dial should be at 99
#[test]
fn test_left_once() {
    let (count, position) = solve_safe_password("L51");
    assert_eq!(count, 0);
    assert_eq!(position, 99);
}

// Use given assignment example as test case
#[test]
fn test_example() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    
    let (count, position) = solve_safe_password(input);
    assert_eq!(count, 3);
    assert_eq!(position, 32);
}


