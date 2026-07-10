// HashMap

use std::collections::HashMap;

fn main() {

    // Method 1
    let mut user1: HashMap<String, u8> = HashMap::new();
    user1.insert(String::from("sanjusabu"), 19);
    println!("{user1:#?}");

    user1.insert("spongebob".to_string(), 20);


    // Method 2
    let user2: HashMap<String, u8> = HashMap::from([(String::from("appu"), 19)]);
    println!("{user2:#?}");


    // For loop method 1
    for (k, v) in &user1 {
        println!("{k} : {v}");
    }

    // For loop method 2
    for key in &user1 {
        println!("{}", key.0);
        println!("{}", key.1);
    }

    println!("Length of user1 variable: {}", user1.len());
    println!("Length of user2 variable: {}", user2.len());

    println!("{:?}", user1.keys());
    println!("{:?}", user1.values());
    println!("{:?}", user1.iter());
}