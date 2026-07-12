// Traits

trait Sound {
    fn sound(&self) -> String {
        String::from("Not declared")
    }
}

struct Animal {
    name: String
}

impl Sound for Animal {
    fn sound(&self) -> String{
        if self.name.to_lowercase() == "dog" {
            "woof".to_string()
        } else {
            "meow".to_string()
        }
    }
}

fn main() {

    let dog = Animal {
        name: String::from("dog"),
    };

    println!("{}",dog.sound());
}