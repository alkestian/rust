struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // if you want any field to be mutable, the entire instance must be mutable
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("randomemail@example.com");

    // we could just create a second struct by manually reusing values from the first instance, OR
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // we can use the struct update syntax to spread the fields:
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // but this would render the user1 variable as unusable, as the values have moved into user2
    // (we could still use the user1.email value though, as it wasn't shifted)

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// we can pass in the variable and explicitly attach them, OR see build_user_implicit
fn build_user_explicit(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// we can implicitly pass the param in, because it shares a name with the struct field
fn build_user_implicit(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// there is also a data type combining tuples and structs - tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// we can also use unit-like structs that don't have any fields
struct AlwaysEqual;
