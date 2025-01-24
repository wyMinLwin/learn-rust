use std::collections::HashMap;
fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Blue"), 20);

    let team_name = String::from("No Team");
    let score = scores.get(&team_name).copied().unwrap_or(0);


    println!("Score: {}",score);

    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(30);
    for (key, value) in scores {
        println!("{key} : {value}");
    }


    let text = "hello world wonderful world";
    let mut text_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", text_map);
    // let field = String::from("String 1");
    // let value = String::from("String 2");

    // let mut map = HashMap::new();
    // map.insert(field, value);
    // field and value are invalid at this point

}
