use rand::RngExt;

fn main() {
    loop {
        let age: u8 = rand::rng().random();

        match age {
            0 => println!("{age}: You are not an adult and how are you even here?"),
            1..=17 => println!("{age}: You are not an adult"),
            18..=99 => println!("{age}: You are an adult"),
            100 => println!("{age}: You are an adult and you hit a century recently"),
            101..=199 => println!("{age}: You are an adult and you're older than 100"),
            200 => println!("{age}: You are an adult and you hit double century recently"),
            201..=250 => {
                println!("{age}: Invalid age for a human being. Are you sure you are a human?")
            }
            _ => {
                println!("{age}: You are weird");
                break;
            }
        }
    }
}
