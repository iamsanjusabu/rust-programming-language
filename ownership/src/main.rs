use std::any::type_name_of_val;

fn main() {
    {
        let whoami = String::from("Sanju Sabu");

        let name: &String = &whoami;

        println!("{}", type_name_of_val(&name));

        // dereferenced
        println!("{}", type_name_of_val(&*name));
    }

    let name: String = String::from("Spongebob SquarePants");
    let name_copy: &String = &name;

    // Dereferencing here makes it the same type (String, String), if we dont do it; (String, &String)
    assert_eq!(name, *name_copy);

    // anyway
    // different types of assert!();
    // assert!();
    // assert_eq!();
    // assert_ne!();
}
