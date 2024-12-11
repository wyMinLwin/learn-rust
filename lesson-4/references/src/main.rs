fn main() {
    let s1 = String::from("Hello");
    let mut s2 = String::from("Hello");
    //by using pointer reference, we can pass the value of s1 to the function without moving the ownership
    let len = cal_len(&s1);
    println!("The length of '{}' is {}", s1, len);

    change(&mut s2);
    println!("The changed value is {}", s2);
}

fn cal_len(s: &String) -> usize {
    s.len()
}
 
fn change(s: &mut String) {
    s.push_str(", World");
}