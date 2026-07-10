// Vec

fn main() {
    // Manual

    let mut users_manual: Vec<String> = Vec::new();

    users_manual.push(String::from("spongebob"));
    users_manual.push(String::from("patrick"));

    users_manual.push("squidward".to_string());
    users_manual.push("mr.krabs".to_string());

    println!("{users_manual:?}");

    // Using macro

    let mut users_macro = vec![
        String::from("spongebob"),
        String::from("patrick"),
        "squidward".to_string(),
        "mr.krabs".to_string(),
    ];

    println!("{users_macro:?}");

    let mut users = users_manual.clone();

    // Removes the last element in the array
    users.pop();

    // Removes the provided index element
    users.remove(1);

    // Appends an array to another (moves ownership of the array being appended)
    users.append(&mut users_macro);

    // becomes: ["spongebob", "squidward", "spongebob", "patrick", "squidward", "mr.krabs"]
    // println!("{:?}", users);

    // becomes: []
    // println!("{:?}", users_macro);

    // Extends an array from another (doesnt move the ownership like append does)
    users.extend(users_manual.clone());

    // output: ["spongebob", "squidward", "spongebob", "patrick", "squidward", "mr.krabs", "spongebob", "patrick", "squidward", "mr.krabs"]
    println!("{users:?}");

    // output: ["spongebob", "patrick", "squidward", "mr.krabs"]
    println!("{users_manual:?}");

    // Checks whether an array is empty or not (returns boolean)
    println!("{}", users.is_empty());

    // Safe indexing
    // Rather than doing .[0] kind of indexing, using .get() is much safer since it returns an enum or option type

    // let second_user = &users_manual[1];  // bad

    let second_user = users_manual.get(1);

    if let Some(name) = second_user {
        println!("{name}");
    } else {
        println!("Index out of bounds");
    }
    
}
