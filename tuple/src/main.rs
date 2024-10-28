/* fn main() {
    let tup: (i32, f64, u8) = (500,6.4,1);
    println!("{},{},{}",tup.0,tup.1,tup.2);
} */

/* fn main() {
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("the value of y is: {}", y);
}
 */

fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}