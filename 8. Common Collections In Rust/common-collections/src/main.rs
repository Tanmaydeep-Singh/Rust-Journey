use std::io;

fn main() {
    let mut v = Vec::new();

    loop {
        // Create a new mutable String to store the user input
        let mut input = String::new();
        let mut vector_input = String::new();
        // Ask the user to enter a number
        println!("Please enter a positive integer (enter 0 to exit):");

        // Read the user input and store it in the 'input' variable
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Parse the input to u32
        let number: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        if number  == 1 {
            println!("Enter the value to be pushed in vector: \n");
            io::stdin().read_line(&mut vector_input).expect("Failed to read line");
            let vector_number: u32 = match vector_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    continue;
                }
            };
            v.push(vector_number);
        }

        if number == 2 {
            println!("Your vector array is:");

            for i in &v {
                print!("{ i }");
            }
        }



        // Check if the number is zero
        if number == 0 {
            break; // Exit the loop if the number is 0
        }

        // Display the user's input
    }

    println!("Goodbye!");
}
