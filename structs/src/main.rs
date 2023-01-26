struct User {
    active: bool,
    username: String,
    email: String,
    password: String,
}

fn build_user(email: String, username: String, password: String) -> User {
    User {
        active: true,
        username,
        email,
        password,
    }
}

fn main() {
    let mut user1 = build_user(
        String::from("jjstyle@yahoo.com"),
        String::from("JJStlye"),
        String::from("password1243")
        );

    let user2 = User {
        username: String::from("Hannah Montana"),
        ..user1
    };

    println!("{}", user1.username);
}
