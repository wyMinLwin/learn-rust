use crate::battle::heroes::Batman;

pub mod battle;

fn main() {
    let batman = Batman { iq: 200 };

    println!("Batman's IQ: {}", batman.iq);
    batman.fight();
}
