fn main() {
    let _s1 = gives_ownership();

    let s2 = String::from("hello");

    let _s3 = takes_and_gives_back(s2);

}

fn gives_ownership() -> String {

    String::from("hello")
    
}

fn takes_and_gives_back(a_string: String) -> String {

    a_string
}