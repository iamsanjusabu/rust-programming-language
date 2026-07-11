fn main() {

    // Panic (A way of exiting the program)
    pass_me_a_number(1);
}

fn pass_me_a_number(num: i32) {
    if num == 1 {
        panic!("Don't pass in 1!");
    }
}