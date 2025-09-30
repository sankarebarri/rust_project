fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("{} {} {} {}", user1.username, user1.email, user1.active, user1.sign_in_count);
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);

    let user = build_user("me@example.com", "me");
    // println!("{user}");

    // Creating Instances from Other Instances with Struct Update Syntax
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anotheremail@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("user3@email.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Point(x, y, z) = origin;

    let subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sing_in_count: 1,
    }
}

// Using the Field Init Shorthand
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;