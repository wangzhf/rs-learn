fn main() {
    let mut x = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);

    // shadowing
    let y = 1;
    let y = y + 1;
    let y = y + 1;
    println!("y is {}", y);
}
