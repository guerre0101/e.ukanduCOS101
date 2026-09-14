use std::io;

fn main(){
    println!("Enter lower boundary");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let lower_boundary:i32 = input1.trim().parse().expect("Failed to input");

    println!("Enter upper boundary");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let upper_boundary:i32 = input2.trim().parse().expect("Failed to input");

    for x in lower_boundary..upper_boundary{    
        println!("Count level is {}", x );
    }
}
