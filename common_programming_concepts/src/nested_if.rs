fn main() {
    let age = 19;

    if age >= 18 {
        if age == 18 {
            println!("You became an adult recently");
        } else if age == 100 {
            println!("You hit the century!!!");
        } else if age > 100  && age <= 120 {
            println!("You probably are one of the oldest person on this planet");
        } else if age > 120 {
            println!("You maybe be special");
        } else {
            println!("You are an adult");            
        }
    } else {
        if age == 0 {
            println!("You just born");
        } else if age < 0 {
            println!("You are not born yet");
        } else {
            println!("You are not an adult");
        }
    }
}
