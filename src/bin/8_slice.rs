fn main() {
    slice_string();
    slice_array();
}

fn slice_string() {
    let helloworld = String::from("hello world");

    let word = first_word(&helloworld[0..6]); // reference to a part of the string from index 0 to 6
    println!("First word is: {}", word);

    let word = first_word(&helloworld[..]); // reference to the whole string
    println!("First word is: {}", word);
    
    let word = first_word(&helloworld);
    println!("First word is: {}", word); // can reference the whole string directly
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn slice_array() {
    let a = [1, 2, 3, 4, 5];
    println!("Array is: {:?}", a);

    let slice = &a[1..3]; // slice of the array from index 1 to 3 (excluding 3)
    println!("Slice is: {:?}", slice);
}