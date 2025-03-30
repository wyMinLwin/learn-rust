use std::thread;
use std::time::Duration;

fn simulate_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generat_workout(intensity: u32, random_number: u32) {
    let expensive_closure = | num | {
        println!("Calculatining slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Today, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

fn main() {
    let simulated_intensity = 32;
    let simulated_random_number = 8;
    generat_workout(simulated_intensity, simulated_random_number);
}
