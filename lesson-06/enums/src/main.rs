#[derive(Debug)]

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum APIResponse {
    Success(String),
    Error(String),
}

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));
    println!("four {:?}, six {:?}", four, six);

    let addr1 = IpAddr {
        kind: IpAddrKind::V4(127, 0, 0, 1),
        address: String::from("100.00.00.000"),
    };

    println!("Your IP{:?} address is {}", addr1.kind, addr1.address);

    let response = APIResponse::Success(String::from("Successfully fetched data"));
    let response2 = APIResponse::Error(String::from("Failed to fetch data"));

    // match response {
    //     APIResponse::Success(data) => {
    //         println!("Message: {}", data);
    //     }
    //     APIResponse::Error(data) => {
    //         println!("Message: {}", data);
    //     }
    // }

    match response2 {
        APIResponse::Success(data) => {
            println!("Message: {}", data);
        }
        APIResponse::Error(data) => {
            println!("Message: {}", data);
        }
    }

    let message = match response {
        APIResponse::Success(data) => data,
        APIResponse::Error(data) => data,
    };
    println!("Message: {}", message);

    let some_number = Option::Some(5);
    let none_number: Option<i32> = Option::None;

    match some_number {
        Option::Some(num) => println!("Number: {}", num),
        Option::None => println!("No Number"),
    }

    match none_number {
        Option::Some(num) => println!("Number: {}", num),
        Option::None => println!("No Number"),
    }
}
