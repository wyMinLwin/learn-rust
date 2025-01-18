enum Stand {
    Figher(String),
    Detective(String),
    Magician(String),
    God(String),
}

fn main() {
    let mut my_collection = Vec::new();
    my_collection.push(1);
    my_collection.push(2);

    let first_item: &i32 = &my_collection[0];
    println!("First item: {}", first_item);

    let second_item: Option<&i32> = my_collection.get(1);
    match second_item {
        Some(value) => println!("Second item: {}", value),
        None => println!("No second item"),
    }

    let crusaiders = vec![
        Stand::Figher(String::from("Star Platinum")),
        Stand::Detective(String::from("Hermit Purple")),
        Stand::Magician(String::from("Magician's Red")),
        Stand::God(String::from("The World")),
    ];

    for stand in &crusaiders {
        match stand {
            Stand::Figher(name) => println!("Owner: Jotaro Kujo, Stand: {}", name),
            Stand::Detective(name) => println!("Owner: Joseph Joestar, Stand: {}", name),
            Stand::Magician(name) => println!("Owner: Avdol, Stand: {}", name),
            Stand::God(name) => println!("Owner: DIO, Stand: {}", name),
        }
    }
        
}
