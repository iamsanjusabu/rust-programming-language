#[allow(unused_variables)]
fn main() {
    // List
    println!("List");

    let bikini_bottom: [String; 4] = [
        String::from("Spongebob SquarePants"),
        String::from("Patrick Star"),
        String::from("Squidward"),
        String::from("Mr. Krabs"),
    ];

    println!("{bikini_bottom:?}");

    let [sponge, starfish, octopus, crab] = &bikini_bottom;

    // shadowing
    let sponge: &String = &bikini_bottom[0];
    let [.., ref red_crab] = bikini_bottom;

    println!("{bikini_bottom:?}");
}
