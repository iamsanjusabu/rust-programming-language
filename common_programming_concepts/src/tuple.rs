#[allow(unused_variables)]
fn main() {
    // Tuple
    
    println!("Tuple");

    let user: (&str, u8, char, String) = ("sanju", 19, 'M', String::from("sanjusabu@icloud.com"));

    
    // print out all the values at once
    println!("{user:?}");

    
    // Print out each values individually
    println!("{} : {} : {} : {}", user.0, user.1, user.2, user.3);
    

    // destructuring a tuple
    let (name, age, gender, ..) = user;
    
    println!("{} : {} : {} : {}", user.0, user.1, user.2, user.3);

    let (.., ref email) = user;
    let email: &String = &user.3;    
}

