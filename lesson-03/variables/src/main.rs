fn main() {

    //variables are immutable by default
    let mut x = 0;
    println!("Before mutate x: {x}");
    x = 1;
    println!("After mutate x: {x}");

    //constants are always immutable
    const THREE_HOUR_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOUR_IN_SECONDS}");

    //shadowing
    let y = 1;
    let y = y + 1;
    println!("shadow y: {y}");
    {
        let y = y * 2;
        println!("shadow y: {y}");
    }
    println!("shadow y: {y}");

    //shadowing can change the type of the variable
    //but mutability cannot
    let greet = "Hello World!";
    println!("greet: {greet}");
    let greet = 1;
    println!("greet: {greet}");
}
