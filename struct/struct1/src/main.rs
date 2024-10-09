struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {

    let mut user1 = User {                                  // Could be mutable
        active : true,
        username: String::from("Leobriand50"),
        email: String::from("leo.briand@smile.fr"),
        sign_in_count: 1,
    };


    // Settings user2 partially equals to user1

    let user2 = User {
        email: String::from("jojo.briand@smile.fr"),
        ..user1
    };

    // or 

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };


    user1.email = String::from("leobriand50@gmail.com");    // Modifiy user1 email

    
}

fn create_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}