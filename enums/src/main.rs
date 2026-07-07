#[allow(dead_code)]
enum Animal {
    Cat,
    Dog,
    Cow
}

#[allow(dead_code)]
#[derive(PartialEq)]
enum Sound {
    Woof,
    Meow,
    Moo
}

impl Sound {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Meow => "meow",
            Self::Woof => "woof",
            Self::Moo => "moo" 
        }
    }
}

impl Animal {
    fn goes(&self, sound: Sound) {
        let sound_as_string: &str = sound.as_str();
        match self {
            Animal::Cat => {
                if sound == Sound::Meow {
                    println!("Meow!!!");
                } else {
                    println!("A cat doesnt {sound_as_string} it meow");
                }
            },
            Animal::Dog => {
                if sound == Sound::Woof {
                    println!("Woof Woof");
                } else {
                    println!("A dog doesnt {sound_as_string} it woof");
                }
            },

            Animal::Cow => {
                if sound == Sound::Moo {
                    println!("Moooooo");
                } else {
                    println!("A cow doesnt {sound_as_string} it moo");
                }
            }
        }
    }
}


fn main() {

    let orange_cat = Animal::Cat;

    orange_cat.goes(Sound::Moo);
}

