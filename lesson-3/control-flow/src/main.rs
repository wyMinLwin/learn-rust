fn main() {
    let x = 5;
    if x > 5 {
        println!("x is greater than 5");
    } else if x < 5 {
        println!("x is less than 5");
    }  else  {
        println!("x is 5");   
    }

    let condition = true;
    let age = if condition { 21 } else { 20 };
    println!("Age is {}", age);
}
