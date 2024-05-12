use std::io;

fn main() {
    loop {
        let mut input = String::new();
        println!("Enter the amount of SOL to transfer: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        let num: u64 = match input.trim().parse::<u64>() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        println!("You entered: {}", num);
        break;
    }
}
