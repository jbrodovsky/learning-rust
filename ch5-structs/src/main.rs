struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1: User = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };

    let mut user2: User = User {
        active: false,
        username: String::from("user2"),
        email: String::from("user2@example.com"),
        sign_in_count: 0,
    };

    user2.email = String::from("anewemail@mydomain.com");
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}