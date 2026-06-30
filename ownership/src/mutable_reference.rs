fn main() {
    let mut a: String = String::from("Rust");

    {
        let b: &mut String = &mut a;
        b.push_str(" programming");
        println!("{b}");
    }

    let c: &mut String = &mut a;
    c.push_str(" language");
    println!("{c}");

    // Address and stuff (must use :p or will get an array of numbers for something.as_bytes())
    println!("{:p}\n{:p}", c.as_bytes(), c.as_ptr());

    
}
