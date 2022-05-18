fn main() {
    // Rust floats f64 precision traps
    let x = 0.1 + 0.2;
    let y: f32 = 0.1 + 0.2;
    println!("{}", 0.1 + 0.2);
    println!("{}", x);
    println!("{}", y);
}
