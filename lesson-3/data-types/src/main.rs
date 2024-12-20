fn main() {
    let guess: u32 = "25".parse().expect("Not a number!");
    println!("guess: {}", guess);

    //interger-overflows
    // let mut x: u8 = 255;
    // x = x + 1;
    // println!("z: {}", x);

    let crab :char = '🦀';
    println!("crab: {}", crab);

    //tuples
    let tup: (char, u8, bool) = ('🔥', 21, true);
    println!("The First Item of Tuple: {}", tup.0);

    //spread tuple
    let (a, b, c) = tup;
    println!("{} {} {}", a, b, c);

    //array
    let array = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let a: [i32: 5] = [1, 2, 3, 4, 5];


    let b = [3; 5];
    // b = [3, 3, 3, 3, 3]
}
