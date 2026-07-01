use std::any::type_name_of_val;

struct User {
    id: u64,
    username: String,
    age: u8,
    email: String,
}

#[derive(PartialEq, Debug)]
struct Nobody(i32);

#[derive(PartialEq, Debug)]
struct Somebody(i32);

fn main() {
    let mut user1: User = User {
        id: 1,
        username: String::from("ladiesman217"),
        age: 19,
        email: String::from("sanjusabu@icloud.com"),
    };

    // println!("
    // id: {}
    // username: {}
    // age: {}
    // email: {}
    // ", user.id, user.username, user.age, user.email);

    // USER 1
    println!("User 1: ");
    println!(
        "id: {}\nusername: {}\nage: {}\nemail: {}",
        user1.id, user1.username, user1.age, user1.email
    );

    user1.username = String::from("iamsanjusabu");

    println!("\nUser 1 updated: ");
    println!(
        "id: {}\nusername: {}\nage: {}\nemail: {}",
        user1.id, user1.username, user1.age, user1.email
    );

    // USER 2
    println!("\nUser 2: ");
    let user2: User = {
        create_user(
            2,
            "sanju".to_string(),
            19,
            "sanjusabu.dev@gmail.com".to_string(),
        )
    };

    println!(
        "id: {}\nusername: {}\nage: {}\nemail: {}",
        user2.id, user2.username, user2.age, user2.email
    );

    // USER 3

    println!("\nUser 3: ");
    let user3 = User {
        id: 3,
        username: "sanju".to_string(),
        ..user1
    };

    println!("\nUser 3: ");

    println!(
        "id: {}\nusername: {}\nage: {}\nemail: {}",
        user3.id, user3.username, user3.age, user3.email
    );

    println!("{}", type_name_of_val(&user2));

    // extras
    let a = Nobody(10);
    let b = Somebody(10);

    // This doesnt work because of different types
    // assert_eq!(a, b);

    assert_eq!(a, a);
    assert_eq!(b, b);
}

fn create_user(id: u64, username: String, age: u8, email: String) -> User {
    User {
        id,
        username,
        email,
        age,
    }
}
