use std::fmt::Display;
use indoc::formatdoc;

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

impl<T: Display, U: Display> Point<T, U> {
    fn print(&self) {
        println!("x: {}\ny: {}", self.x, self.y);
    }

    fn print_with<V: Display, W: Display>(&self, var2: &Point<V, W>) {
        println!(
            "{}",
            formatdoc!(
                "
                -----SELF-----
                x: {}
                y: {}

                -----VAR2-----
                x: {}
                y: {}
                ", self.y, self.x, var2.x, var2.y
            )
        );
    }
}

fn main() {
    let p1 = Point {
        x: 10,
        y: 40
    };

    let p2 = Point {
        x: String::from("sanju"),
        y: String::from("sabu")
    };

    println!("P1: {p1:?}");
    println!("P2: {p2:?}");

    println!("print()");
    p1.print();
    p2.print();

    println!("print_with()");
    p1.print_with(&p2);
}