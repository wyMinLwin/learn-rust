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


    //loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result is {}", result);


    let mut count = 0;
    'counting_up: loop {
        println!("Counting up {}", count);
        let mut remaining = 10;
        loop {
            println!("Remaining {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
}
