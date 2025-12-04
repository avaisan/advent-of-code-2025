struct Rectangle {
    width: u32,
    height: u32,
}

struct Cube {
    width: u32,
    height: u32,
    length: u32,
}
// define method in struct
// implementation block contains methods associated with struct
// here it is function "area"
// point of method is to put all things we can do with struct inside impl block
// then we can call methods on instances of struct, not having to find functions that take struct as parameter
// -> cleaner code
// impl block is like a namespace for struct
// maybe like a class in OOP?
// https://www.reddit.com/r/rust/comments/qg1eov/how_are_classes_and_impls_different_how_are_impls/

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// struct can have multiple impl blocks
impl Cube {
    fn volume(&self) -> u32 {
        self.width * self.height * self.length
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let cube1 = Cube {
        width: 30,
        height: 50,
        length: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "The volume of the cube is {} cubic pixels.",
        cube1.volume()
    );
}