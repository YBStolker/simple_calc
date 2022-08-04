mod components;

use components::eval::eval;

pub fn run() {
    println!("Simply calcing...");
    println!("Type 'exit' to exit.");
    loop {
        println!("Give your input:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line for some reason");

        if input.trim() == "exit" {
            break;
        }

        match eval(&input) {
            Ok(num) => println!("> {}", num),
            Err(e) => println!("error: {}", e),
        };
    }
}
