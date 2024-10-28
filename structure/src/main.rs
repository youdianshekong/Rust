/* fn main() {
    println!("Hello, world!");
} */
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: True,
    sign_in_count: 1,
}

user1.email = String::from("anotheremail@example.com");
