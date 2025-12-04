fn main() {
    let width1 = 10;
    let height1 = 20;
    let length1 = 30;

    println!(
        "The volume of the cube is {} cubic pixels.",
        volume(width1, height1, length1)
    );

    struct_calculate_example();
}

fn volume(width: u32, height: u32, length: u32) -> u32 {
    width * height * length
}

// using struct to group related data together
struct Cube {
    width: u32,
    height: u32,
    length: u32,
}

fn struct_calculate_example() {
    let cube1 = Cube {
        width: 10,
        height: 20,
        length: 30,
    };

    println!(
        "The volume of the cube is {} cubic pixels.",
        volume2(&cube1)
    );
}

fn volume2(cube: &Cube) -> u32 {
    cube.width * cube.height * cube.length
}