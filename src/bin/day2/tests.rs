use super::*;

/*
For example:

```
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
```

The ranges are separated by commas (,); each range gives its first ID and last ID separated by a dash (-).

You can find the invalid IDs by looking for any ID which is made only of some sequence of digits repeated twice. 

So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.

None of the numbers have leading zeroes; 0101 isn't an ID at all. (101 is a valid ID that you would ignore.)

Find all of the invalid IDs that appear in the given ranges. In the above example:

11-22 (range: 11,12,13,...20,21,22) has two invalid IDs, 11 and 22 .
95-115 has one invalid ID, 99.
998-1012 has one invalid ID, 1010.
1188511880-1188511890 has one invalid ID, 1188511885.
222220-222224 has one invalid ID, 222222.
1698522-1698528 contains no invalid IDs.
446443-446449 has one invalid ID, 446446.
38593856-38593862 has one invalid ID, 38593859.
*/


#[test]
fn test_single_digit_repeated() {
    assert_eq!(is_invalid(11), true);
    assert_eq!(is_invalid(22), true);
    assert_eq!(is_invalid(99), true);
}

#[test]
fn test_two_digit_repeated() {
    assert_eq!(is_invalid(6464), true);
    assert_eq!(is_invalid(1010), true);
}

#[test]
fn test_three_digit_repeated() {
    assert_eq!(is_invalid(123123), true);
}

#[test]
fn test_valid_numbers() {
    assert_eq!(is_invalid(10), false); // Halves don't match
    assert_eq!(is_invalid(999), false);  // Odd length
    assert_eq!(is_invalid(1234), false); // Halves don't match
}

#[test]
fn test_sum_of_invalid_ids_one_invalid() {
    let input = "95-115"; // 99
    assert_eq!(invalid_ids(input).iter().sum::<i64>(), 99);
}


#[test]
fn test_sum_of_invalid_ids_two_invalids() {
    let input = "11-22"; // 11 + 22 = 33
    assert_eq!(invalid_ids(input).iter().sum::<i64>(), 33);
}

#[test]
fn test_example() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    assert_eq!(invalid_ids(input).iter().sum::<i64>(), 1227775554);
}
