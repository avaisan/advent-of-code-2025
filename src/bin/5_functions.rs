fn main() {
    println!("Hello world!");

    another_function(5);
    convert_kmh_to_minkm(15, "min/km"); // unit here is just a second function parameter, defined as string
    
    let y = plus_one(5);
    println!("The value of y plus one is: {y}");
}

fn another_function(x: i32) {
    println!("Second function!");
    println!("The value of x is: {x}");
}

fn convert_kmh_to_minkm(x: i32, unit: &str) {
    let x = 60 / x;
    println!("15 km/h is the same as {x} {unit}");
}


// return value. No need to use return keyword, but return type needs to be defined after ->
fn plus_one(y: i32) -> i32 {
    y + 1
}