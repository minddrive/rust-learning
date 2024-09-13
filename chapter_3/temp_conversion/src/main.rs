use std::io;

fn main() {
    let done = false;

    while !done {
        println!("What temperature do you want to convert?");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("Is this Fahrenheit (F) or Celsius (C)?");

        let mut temp_type = String::new();

        io::stdin()
            .read_line(&mut temp_type)
            .expect("Failed to read line");

        let temp_type: String = match temp_type.trim().parse() {
            Ok(temp_type) => temp_type,
            Err(_) => {
                println!("Please type a letter!");
                continue;
            }
        };

        if temp_type == "F" {
            println!("{}F is equivalent to {}C", temp, fahrenheit_to_celsius(temp));
        } else if temp_type == "C" {
            println!("{}C is equivalent to {}F", temp, celsius_to_fahrenheit(temp));
        } else {
            println!("You need to choose 'F' or 'C', starting over!");
        }

        println!("Do you want to do another temperature?");

        let mut another_temp = String::new();

        io::stdin()
            .read_line(&mut another_temp)
            .expect("Failed to read line");

        let another_temp: String = match another_temp.trim().parse() {
            Ok(another_temp) => another_temp,
            Err(_) => {
                println!("Please type a letter!");
                continue;
            }
        };

        if another_temp != "Y" {
            break;
        }
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}
