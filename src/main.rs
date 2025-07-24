// fn main() {
//     println!("Hello, world!");
// }

// Option Example
// fn main() {
//     let names = vec!["Alice", "Bob", "Carol"];
//     let maybe_name = names.get(1);

//     match maybe_name {
//         Some(name) => println!("Found: {}", name),
//         None => println!("No name found at this index."),
//     }
// }

// Result Example
// fn divide(a: f64, b: f64) -> Result<f64, String> {
//     if b == 0.0 { Err("Cannot divide by zero".to_string()) } else { Ok(a / b) }
// }

// fn main() {
//     match divide(10.0, 0.0) {
//         Ok(result) => println!("Result: {}", result),
//         Err(msg) => eprintln!("Error: {}", msg),
//     }
// }

// Matching on Result
// use std::fs::read_to_string;

// fn main() {
//     let result = read_to_string("data.txt");

//     match result {
//         Ok(content) => println!("Content:\n{}", content),
//         Err(error) => eprintln!("Read error: {}", error),
//     }
// }

// unwrap() — Use with Caution
// let some_value = Some(10);
// let value = some_value.unwrap(); // ✅ OK

// let none_value: Option<i32> = None;
// let value = none_value.unwrap(); // ❌ Panics: called `Option::unwrap()` on a `None` value

// let result: Result<i32, &str> = Err("Oops");
// let value = result.unwrap(); // ❌ Panics with "Oops"

// let file = std::fs::read_to_string("config.toml")
//     .expect("Failed to read config file");

// use std::fs::read_to_string;

// fn read_file(path: &str) -> Result<String, std::io::Error> {
//     let content = read_to_string(path)?; // If this fails, function returns Err automatically
//     Ok(content)
// }

// fn first_char(s: &str) -> Option<char> {
//     let first = s.chars().next()?;
//     Some(first)
// }

// Converting Between Option and Result

// let maybe_val = Some("hello");
// let result: Result<&str, &str> = maybe_val.ok_or("Value was None");

// let result: Result<i32, &str> = Ok(42);
// let option = result.ok(); // Some(42)

// Example 1: Parsing User Input into an Integer
// use std::io::{ self, Write };

// fn main() {
//     print!("Enter a number: ");
//     io::stdout().flush().unwrap(); // Ensure prompt is printed

//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read line");

//     match input.trim().parse::<i32>() {
//         Ok(num) => println!("You entered: {}", num),
//         Err(_) => eprintln!("Invalid number!"),
//     }
// }

// Example 2: Reading a Config File and Extracting a Value
// use std::fs::read_to_string;

// fn read_port_from_config(path: &str) -> Result<u16, Box<dyn std::error::Error>> {
//     let contents = read_to_string(path)?;
//     for line in contents.lines() {
//         if let Some(port_str) = line.strip_prefix("port=") {
//             let port = port_str.trim().parse::<u16>()?;
//             return Ok(port);
//         }
//     }
//     Err("port not found in config".into())
// }

// fn main() {
//     match read_port_from_config("config.txt") {
//         Ok(port) => println!("Configured port: {}", port),
//         Err(e) => eprintln!("Error: {}", e),
//     }
// }

// 📤 Example 3: Custom Error Type with Result
fn validate_username(username: &str) -> Result<(), String> {
    if username.len() < 4 {
        Err("Username must be at least 4 characters.".to_string())
    } else if username.contains(' ') {
        Err("Username must not contain spaces.".to_string())
    } else {
        Ok(())
    }
}

fn main() {
    match validate_username("ab") {
        Ok(_) => println!("Valid username."),
        Err(e) => eprintln!("Validation failed: {}", e),
    }
}
