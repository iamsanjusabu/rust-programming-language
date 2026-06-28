#[allow(unused_variables)]
fn main() {
    // List
    println!("List");

    let family: [String; 4] = [String::from("sanju"), String::from("sahil"), String::from("shaila"), String::from("sabu")];

    println!("{family:?}");

    let [me, brother, mother, father] = &family;
    let me: &String = &family[0];

    let [.., ref father] = family;

    println!("{family:?}");
}


