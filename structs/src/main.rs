#[derive(Debug, PartialEq)]
struct Rectangle {
    length: i32,
    breadth: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.length * self.breadth
    }

    fn equal(&self, obj2: &Rectangle) -> bool {
        self.length == obj2.length && self.breadth == obj2.breadth
    }

    fn can_hold(&self, obj2: &Rectangle) -> bool {
        self.length > obj2.length && self.breadth > obj2.breadth
    }
}

impl Rectangle {
    fn square(size: i32) -> Rectangle {
        Rectangle {
            length: size,
            breadth: size,
        }
    }
}

fn main() {
    let rect1: Rectangle = Rectangle {
        length: 10,
        breadth: 5,
    };

    let rect2: Rectangle = Rectangle {
        length: 20,
        breadth: 10,
    };

    println!("Rectangle 1: {rect1:?}\nRectangle 2: {rect2:?}");

    println!("{}", rect1.area());
    println!("{}", rect2.area());

    println!("Rectangle 1 == Rectangle 2: {}", rect1.equal(&rect2));
    println!(
        "Rectangle 1 can hold Rectangle 2: {}",
        rect1.can_hold(&rect2)
    );

    let rect3: Rectangle = Rectangle::square(10);
    println!("Rectangle 3 created using Rectangle::square(10): {rect3:?}");

    // this will fail
    assert_eq!(rect1, rect2);
}
