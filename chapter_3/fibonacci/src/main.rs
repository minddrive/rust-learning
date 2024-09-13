use std::io;

fn main() {
    loop {
        println!("Which Fibonacci number do you want?");

        let mut nth_fib = String::new();

        io::stdin()
            .read_line(&mut nth_fib)
            .expect("Failed to read line");

        let nth_fib: i64 = match nth_fib.trim().parse() {
            Ok(nth_fib) => nth_fib,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("Fibonacci number {} is: {}", nth_fib, fibonacci(nth_fib));

        println!("Calculate another Fibonnaci number?");

        let mut another_fib = String::new();

        io::stdin()
           .read_line(&mut another_fib)
            .expect("Failed to read line");

        let another_fib: String = match another_fib.trim().parse() {
            Ok(another_fib) => another_fib,
            Err(_) => {
                println!("Did not type 'Y' or 'N', assuming 'N'");
                break;
            }
        };

        if another_fib != "Y" {
            break;
        }
    }
}

fn fibonacci(num: i64) -> i64 {
    let mut fibonacci: i64 = 0;

    if num == 0 {
        0
    } else if num == 1 {
        1
    } else {
        let mut prev: i64 = 0;
        let mut curr: i64 = 1;
        for _ in 2..=num {
            fibonacci = prev + curr;
            prev = curr;
            curr = fibonacci;
        }

        fibonacci
    }
}
