use std::io;
 
// Takes a paramater, reads a number, and returns it as f64.
// Every shape function reuses this, so we write the input code only once.
fn read_number(prompt: &str) -> f64 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let value: f64 = input.trim().parse().expect("Invalid input");
    return value;
}
 
// Reads the menu choice as a whole number
fn read_choice() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let choice: i32 = input.trim().parse().expect("Invalid input");
    return choice;
}
 
fn get_pi() -> f64 {
    let a: f64 = 22.0;
    let b: f64 = 7.0;
    let c: f64 = a / b;
    return c;
}
 
// Takes the input for each shape, calculates, then returns the answer
 
fn trapezium_area() -> f64 {
    let height: f64 = read_number("Enter the height:");
    let base1: f64 = read_number("Enter base 1:");
    let base2: f64 = read_number("Enter base 2:");
    return height / 2.0 * (base1 + base2);
}
 
fn rhombus_area() -> f64 {
    let diagonal1: f64 = read_number("Enter diagonal 1:");
    let diagonal2: f64 = read_number("Enter diagonal 2:");
    return 0.5 * diagonal1 * diagonal2;
}
 
fn parallelogram_area() -> f64 {
    let base: f64 = read_number("Enter the base:");
    let altitude: f64 = read_number("Enter the altitude:");
    return base * altitude;
}
 
fn cube_surface_area() -> f64 {
    let side: f64 = read_number("Enter the side length:");
    return 6.0 * side * side;
}
 
fn cylinder_volume() -> f64 {
    let radius: f64 = read_number("Enter the radius:");
    let height: f64 = read_number("Enter the height:");
    return get_pi() * radius * radius * height;
}
 
// this is the main menu, where the user reads and selects the shape they want to use
 
fn main() {
    println!("Please choose from one of the five shapes below");
    println!("1. Trapezium (area)");
    println!("2. Rhombus (area)");
    println!("3. Parallelogram (area)");
    println!("4. Cube (surface area)");
    println!("5. Cylinder (volume)");
    println!("Enter your choice (1-5):");
 
    let choice: i32 = read_choice();

    // this is why we needed the choice to be read as a whole number
    // you could also use f64 but that dosent look nice
 
    if choice == 1 {
        let result = trapezium_area();
        println!("Area of the trapezium is: {}", result);
    } else if choice == 2 {
        let result = rhombus_area();
        println!("Area of the rhombus is: {}", result);
    } else if choice == 3 {
        let result = parallelogram_area();
        println!("Area of the parallelogram is: {}", result);
    } else if choice == 4 {
        let result = cube_surface_area();
        println!("Surface area of the cube is: {}", result);
    } else if choice == 5 {
        let result = cylinder_volume();
        println!("Volume of the cylinder is: {}", result);
    } else {
        println!("Invalid choice. Please run the program again and pick 1 to 5.");
    }
}