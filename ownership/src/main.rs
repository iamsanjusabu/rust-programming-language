fn main() {

    let length = length_till_a_white_space(&"Sanju Sabu".to_string());

    println!("{length}");
}

fn length_till_a_white_space(text: &String) -> usize {
    let bytes = text.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return index;
        }
    }

    return text.len();
}
