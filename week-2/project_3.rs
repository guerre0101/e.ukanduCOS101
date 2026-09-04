fn main() {
    let p: f64 = 210000.0;
    let r: f64 = 0.05;
    let t: f64 = 3.0;

    let a = p * (1.0 - r).powf(t);

    println!("Depreciated Value is {}", a);
}