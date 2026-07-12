fn max<T: PartialOrd>(arr: &[T]) -> &T {
    let mut largest = arr.get(0).unwrap();

    for i in arr {
        if i > largest {
            largest = i;
        }
    }

    largest
}

fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    let largest = max(&numbers);

    println!("Largest item in the array {numbers:?}: {largest}");
}
