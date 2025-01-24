use std::fs::File;
// use std::io::ErrorKind;

fn main() {
    //File::open("greeting.txt").unwrap();

    File::open("greeting.txt").expect("Greeting should be in the file");
    // let greeting_file_result = File::open("greeting.txt");

    // match greeting_file_result {
    //     Ok(file) => {
    //         println!("File opened successfully: {:?}", file);
    //     },
    //     Err(error) => {
    //         match error.kind() {
    //             ErrorKind::NotFound => {
    //                 match File::create("greeting.txt") {
    //                     Ok(file) => {
    //                         println!("File created successfully: {:?}", file);
    //                     },
    //                     Err(file_create_error) => {
    //                         println!("Error creating file: {:?}", file_create_error);
    //                     },
    //                 }
    //             },
    //             other_error => {
    //                 println!("Error opening file: {:?}", other_error);
    //             }
    //         }
    //     },
            
    // }
}
