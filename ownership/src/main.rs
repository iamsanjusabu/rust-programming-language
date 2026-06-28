use std::io::{self, Write};

// Ownership and scope
// Manual drops like i did here is not required, i did it because it was easier for me this way to track things

#[allow(unused_variables)]
fn main() {
    // ** 1 **
    // a String variable with the length and capacity of 10
    let name: String = String::from("Sanju Sabu");

    // Size or length of the string
    println!("{}", name.len());

    // The capacity of the variable
    println!("{}", name.capacity());

    // Manual drop; rust does this automatically
    drop(name);

    // println!("{name}"); // This will throw an error

    // ** 2 **
    // Block scope { code here }
    {
        // Things created inside this block only work here
        let name: String = String::from("Spongebob");
        println!("{name}");
    }

    // Here we cant access the name from the block scope
    // println!("{name}"); // This will throw an error

    // fun fact, this block scope can return to a variable
    let age: u8 = {
        loop {
            let mut input: String = String::new();
            print!("How old are you? ");
            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse() {
                Ok(num) => break num,
                Err(_) => println!("Numbers only"),
            };
        }
    };

    // This much of complexity is not needed to read an input, convert it to integer and save it to a variable. I did it because i can
    println!("You are {age} years old");

    // ** 3 **
    // Ownership change

    // Variables
    let mut greeting = String::from("Hi");
    println!("{greeting}");

    // This overrode the "Hi" value and since the "Hi" value wasnt being used by any other variable (no owners), it got freed automatically from the heap, "drop(old)"
    greeting = String::from("hello");
    println!("{greeting}");

    // To actually append something like a string to a string variable, we use push_str
    greeting.push_str(", world");
    println!("{greeting}");

    drop(greeting);

    // To pass ownership of a variable which lives in heap to another variable
    let str1: String = String::from("Sanju");
    println!("{str1}");

    let str2 = str1;
    println!("{str2}");

    // Cant access "str1" anymore since the ownership of the string "Sanju" has been moved to variable "str2"
    // println!("{str1}"); // This will throw an error
    drop(str2);

    // Borrowing instead of taking ownership
    let str3 = String::from("Confidential");

    // Instead of getting ownership, here im only giving the "str4" variable the reference of "str3"
    let str4 = &str3;
    print!("str3: {str3}\nstr4: {str4}\n");

    // Functions (can be ran outside of a code block too unlike what im doing)
    {
        let str5 = String::from("Sanju");

        takes_ownership(str5);

        // println!("{name}"); // error
    }
} // Variables still in scope are dropped here, in reverse order of creation.
fn takes_ownership(text: String) {
    println!("{text}");
}
