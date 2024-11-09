use std::fs::File;

fn main() {
    let file_result = File::open("nonexistent.txt");

    match file_result {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(error) => println!("Failed to open file: {:?}", error),
    }
}
