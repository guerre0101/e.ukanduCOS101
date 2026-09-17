use std::io;
fn main() {
    let mut input1 = String::new();
let mut input2 = String::new();

// input your age and state whether youre experienced or not
println!("Enter your age");
io::stdin().read_line(&mut input1).expect("Not a valid string");
let age:f32 = input1.trim().parse().expect("Not a valid number");

println!("Are you experienced?");
io::stdin().read_line(&mut input2).expect("Not a valid string");
let experience:bool = input2.trim().to_lowercase() == "yes";

//runs a comparison to allocate the salary

if age >= 40.0 && experience{
    println!("Your salary is N1,560,000");
} else if (30.0..=39.0).contains(&age) && experience{
    println!("Your salary is N1,480,000");
} else if age <= 28.0 && experience{
    println!("Your salary is N1,300,000");
} else{
    println!("Your salary is N100,000");
  }

}



