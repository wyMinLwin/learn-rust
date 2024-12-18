// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }
//
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // let mut user1 = User {
    //     username: String::from("Armin Arlert"),
    //     email: String::from("armin@surveycorps.com"),
    //     sign_in_count: 1,
    //     active: true,
    // };
    // user1.email = String::from("armin@surveycorps.aot");

    // let user2 = build_user(
    //     String::from("levi@surveycorps.aot"),
    //     String::from("Levi Ackerman"),
    // );

    // let user3 = User {
    //     username: String::from("Eren Yeager"),
    //     ..user2
    // };

    // println!("User 3: {}", user3.username);
    // let width1 = 30;
    // let height1 = 50;
    // let area1 = calculate_area(width1, height1);
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let area1 = calculate_area(&rect1);
    println!("The area of rectangle 1 is: {}", area1);
    println!("The area of rectangle is: {}", rect1.get_area());
    println!("Print Rectangle: {rect1:?}");
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }
fn calculate_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
