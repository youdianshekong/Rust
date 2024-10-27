/* fn main() {
    println!("Hello, world!");
} */

/* fn main() {
    let my_name = "Pascal";
    greet(my_name);
}

fn greet(name: String) {
    println!("Hello,{}!", name);
} */

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    &s[..1]
}