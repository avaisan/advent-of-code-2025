use std::io;

fn main() {
    println!("Write something!");

    let mut variable = String::new(); // Mut = mutable variable

    io::stdin()
        .read_line(&mut variable) // & means passing a reference to var variable
        .expect("Failed to read line");

    println!("You wrote: {}", variable);
}