fn main() {
    if_statement(6);
    loop_example(10);
    while_example(5);
    loop_through_elements();
    for_loop();
}

fn if_statement(x: i32) {
    let number = x;

    if number % 4 == 0 {
        println!("number {} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("number {} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("number {} is divisible by 2", number);
    } else {
        println!("number {} is not divisible by 4, 3, or 2", number);
    }
}

// loop to increase counter value until it reaches x
fn loop_example(x: i32) {
    let mut counter = 0;

    let result = loop {
        println!("Counter is at: {counter}");
        counter += 1;

        if counter == x {
            break counter * 1;
        }
    };

    println!("Counter ended at: {result}");
}

// counting down using while
fn while_example(y: i32) {
    let mut number = y;

    println!("Countdown:");
    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("Go!");
}

fn loop_through_elements() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    println!("Looping through array elements: {:?}", a);
    while index < 5 {
        println!("element {} value is: {}", index, a[index]);

        index += 1;
    }
}

fn for_loop() {
    println!("Here is for loop counting down:");
    for number in (1..4).rev() {
        println!("{number}");
    }
}