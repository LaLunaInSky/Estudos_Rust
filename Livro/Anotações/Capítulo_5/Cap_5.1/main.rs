struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user_001 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user_001.email = String::from("anotheremaiil@example.com");

    println!("active = {}\nusername = {}\nemail = {}\nsign_in_count = {}", user_001.active, user_001.username, user_001.email, user_001.sign_in_count);

    //

    // let user_002 = User {
    //     active: user_001.active,
    //     username: user_001.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user_001.sign_in_count,
    // };

    let user_002 = User {
        email: String::from("another@examplel.com"),
        ..user_001
    };

    println!("\nactive = {}\nusername = {}\nemail = {}\nsign_in_count = {}", user_002.active, user_002.username, user_002.email, user_002.sign_in_count);

    //
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    //
    let subject = AlwaysEqual;
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true.
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }