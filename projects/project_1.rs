fn main() {
    let p: f64 = 520000000.0;
    let r: f64 = 0.10;
    let t: f64 = 5.0;

    let a = p * (1.0 + r).powf(t) - p;

    println!("Compound Interest is {}", a);
}