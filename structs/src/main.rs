fn main() {
    let mut user1 = User {
        email: String::from("email@email.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 3,
    };

    let usernameofuser1 = user1.username;
    println!("{}", usernameofuser1);

    //Creating Instances From Other Instances With Struct Update Syntax

    let user2 = User {
        email: String::from("email@email.com"),
        active: user1.active,
        username: String::from("username"),
        sign_in_count: user1.sign_in_count,
    };

    // we can get same result with less code using .. syntax

    let user3 = User {
        email: String::from("email3.email.com"),
        ..user2
    };
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
