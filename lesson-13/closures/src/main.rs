use closures::ShirtColor;
use closures::Inventory;

fn main() {
    let shirt_color = ShirtColor::Red;
    println!("Shirt color: {:?}", shirt_color);

    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue],
    };

    let user_1_preference = Some(ShirtColor::Red);
    let user_2_preference = None;

    let giveaway_1 = store.giveaway(user_1_preference);
    let giveaway_2 = store.giveaway(user_2_preference);
    println!("Giveaway 1: {:?}", giveaway_1);
    println!("Giveaway 2: {:?}", giveaway_2);

}
