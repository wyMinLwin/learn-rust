fn main() {
    println!("Hello, world!");
    another_function(100);
    //expression
    let y = {
        let x = 20;
        x + 1
    };
    println!("The value of y is: {}", y);    
    let result = add(10, 20);
    println!("The result of add function is: {}", result);
}

fn another_function(x: i32) {
    println!("The value pass to function is {}.", x);
}

//function with return value
fn add(x: i32, y: i32) -> i32 {
    x + y
}
