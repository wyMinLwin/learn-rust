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
    'outer_loop: loop {
        let mut remaining = 10;
        println!("count: {}", count);
        loop {
            if count == 2 {
                break 'outer_loop;
            }
            println!("Remaining; {}", remaining);
            if remaining == 9 {
                break;
            }
            remaining -= 1;
        }
        count += 1;
    }

    let mut number = 3;
    while number >= 0 {
        println!("{}!", number);
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {}", element);
    }

    for n in (0..4).rev() {
        println!("{}", n);
    }
}
