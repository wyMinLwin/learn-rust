use std::fmt::Display;

fn main() {
    let winner :&str = announce_and_compare("Neil", "Joe", 1);
    println!("The winner is: {}", winner);
}


fn announce_and_compare<'a,T>(player1: &'a str, player2: &'a str, round: T) -> &'a str 
    where T: Display {
        println!("Announcing the winner of round {}...", round);
        if player1.len() > player2.len() {
            player1
        } else {
            player2
        }
    }