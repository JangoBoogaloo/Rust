fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("xiang@example.com"),
        ..user1
    };
    println!("{0}, {1}", user1.email, user2.email);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}