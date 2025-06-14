#[derive(Debug)]
#[allow(dead_code)]
// above line adds :? or Debug trait
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("User 1s details are {user1:?}");
    // :? trait is the debug trait and allows us
    // to print everything in the string in a useful manner

    /*
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    */
    // above code can be done by:
    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // code below cannot be executeddue to code above
    /*
    user1.email = String::from("anotheremail@example.com");
    if user1.active {
        println!(
            "{}, {}, {}",
            user1.username, user1.email, user1.sign_in_count
        );
    }
    */
}
