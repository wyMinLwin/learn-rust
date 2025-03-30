use std::thread;

fn main() {

    //closure
    let add_one = | x | {x +1};
    let x = 5;
    println!("{} + 1 = {}", x, add_one(x));

    //closure which capture immutable variable
    let list = vec![1,2,3];
    
    //before define closure
    println!("Before define closure: {list:?}");

    let borrow_list = || println!("Closure: {list:?}");

    println!("before call closure: {list:?}");
    borrow_list();
    println!("after call closure: {list:?}");

    //closure which capture mutable variable
    let mut mut_list = vec![4,5,6];
    println!("Before define closure: {:?}", mut_list);
    let mut mut_borrow_list = || mut_list.push(21);
    mut_borrow_list();
    println!("After define closure: {mut_list:?}");

    println!("---------------------------------");
    let thread_list = vec![1,2,3];
    println!("Before define closure: {:?}", thread_list);

    thread::spawn(move || println!("From thread: {:?}", thread_list)).join().unwrap();
}
