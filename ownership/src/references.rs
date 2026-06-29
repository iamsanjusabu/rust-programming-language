fn main() {

    // scope 1
    {
        let name: String = String::from("Sanju Sabu");
        // pointer address of name
        println!("{:p}", name.as_ptr());

        let whoami: String = name;
        // pointer address of whoami (same as name since name passed the ownership of its data to this variable)
        println!("{:p}", whoami.as_ptr());

        // Cant do this
        // println!("{name}");

        // A new fresh variable
        let name: String = String::from("Sanju Sabu");
        // pointer address of the second name variable (a completely new variable so it will have a different pointer address compared to the other variables)
        println!("{:p}", name.as_ptr());
    } // scope of variables inside the code block is over here

    // scope 2
    {
        let name = String::from("Sanju Sabu");

        // Giving ownership of the variable "name"; losses the access to it afterwards
        takes_ownership(name);
    }

    let name: String = String::from("Sanju Sabu");
    takes_reference(&name);
}

// 1
fn takes_ownership(name: String) {
    println!("Took ownership of variable containing the value: {name}");
}

// 2
fn takes_reference(name: &String) {
    println!(
        "Took only the reference of the variable, which means the owner of the variable being passed to this function still owns it; {name}"
    );
}
