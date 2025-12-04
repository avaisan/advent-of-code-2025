fn main() {
    let mut user1 = User {
        active: false,
        username: String::from("avaisan123"),
        email: String::from("avaisan123@email.com"),
        sign_in_count: 165,
    };

    // to mutate a struct field, the whole struct must be mutable (let mut user1). Then we can:
    user1.email = String::from("changedemail@email.com");
    // not possible to set only one field as mutable. The whole struct must be mutable.

    // create a new instance from another instance
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anotheruser@email.com"),
        sign_in_count: user1.sign_in_count,
    };

    // or like this, .. means the remaining fields should have the same value as the fields in given instance (user1)
    let user3 = User {
        email: String::from("thirduser@email.com"),
        ..user1
    };
}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// this is how to define a tuple struct. Used to create different types with the same fields.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // destructure tuple structs to access individual values:
    let Color(r, g, b) = black;
    let Point(x, y, z) = origin;
}