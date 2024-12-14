fn main() {
    let mut s = String::from("Hello world!");
    let index = first_word(&s);
    // s.clear();
    println!("The first word is: {}", index);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
