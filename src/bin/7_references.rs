fn main() {

    reference_example();

    change_mut();
}

fn reference_example() {
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{}", s1); // error, s1 is no longer valid since Rust protects memory by assigning values to only one variable at a time
    println!("s2 is: {}", s2);

    let s3 = String::from("hello");
    let len = calculate_length(&s3); // pass a reference to s3 using &, so ownership is not moved. Thus we can use s3 later.
    println!("The length of '{s3}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_mut() {
    let mut r = String::from("hello");

    change(&mut r); // this works because mutables can be altered through references
    println!("Changed string is: {}", r);

    let r1 = &mut r;
    //let r2 = &mut r; // error, cannot have two mutable references to the same variable at the same time
}

fn change(some_string: &mut String) {
    some_string.push_str(", world"); // note, this does not return anything. It alters the original string through the mutable reference.
}