fn main() {

    // using variables
    let mut x = 5; // have to use mut for the next var x = 6 to work
    println!("The value of x is: {x}");
    x = 6;
    x = x + 1; // does not need mut, this second var overshadows the previous from compiler perspective
    println!("The value of x is: {x}");

    {
        let x = x * 2; // let creates a new variable that shadows the previous x
        println!("The value of x in the inner scope is: {x}");
    }

    // data types
    let y: f32 = 3.0; // f32 = 32 bit floating point number
    let z: f64 = 2.0; // f64 = 64 bit floating point number
    let w: char = 'w'; // char
    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple
    println!("The value of y is: {:.1}, the value of z is: {:.1}, the value of w is: {w}", y,z);
    println!("The value of tuple is: ({}, {}, {})", tup.0, tup.1, tup.2);
}