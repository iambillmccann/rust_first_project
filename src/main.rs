// use std::fs::File;

fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        return Err("Cannot divide by zero");
    }
    Ok(a / b)
}
fn main() {
    let result1 = divide(10, 2);
    match result1 {
        Ok(value) => println!("Result1: {}", value),
        Err(err) => println!("Error1: {}", err),
    }

    let result2 = divide(10, 0);
    match result2 {
        Ok(value) => println!("Result2: {}", value),
        Err(err) => println!("Error2: {}", err),
    }
}
