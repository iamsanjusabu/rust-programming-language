#[allow(unused)]
#[derive(Debug, Clone, Copy)]
enum Shapes {
    Circle,
    Triangle,
    Rectangle,
    Square,
}

fn main() {
    let circle: [Shapes; 5] = [Shapes::Circle; 5];

    println!("{circle:?}")
}
