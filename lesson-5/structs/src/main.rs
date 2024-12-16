struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        username: String::from("Armin Arlert"),
        email: String::from("armin@surveycorps.com"),
        sign_in_count: 1,
        active: true,
    };
    user1.email = String::from("armin@surveycorps.aot");

    let user2 = build_user(
        String::from("levi@surveycorps.aot"),
        String::from("Levi Ackerman"),
    );

    let user3 = User {
        username: String::from("Eren Yeager"),
        ..user2
    };

    println!("User 3: {}", user3.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
