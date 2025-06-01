struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User{
    User { active: true, username, email, sign_in_count: 1 }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername1234"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("yetanotheremail@world.com"),
        username: user1.username.clone(),
        ..user1     // this is struct update syntax (fill rest fields same as in user1)
    };

    let user3 = build_user(String::from("somefunnyemail@serv.com"), String::from("funnyuser"));

    println!("User1 name: {}", user1.username);
    println!("User1 sign_in_count: {}", user1.sign_in_count);
    println!("User2 email: {}", user2.email);
    println!("User3 active: {}", user3.active);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.0);
    println!("{}", origin.1);

    let Point(x,y,z) = origin;

    println!("{}", x);
    
}
