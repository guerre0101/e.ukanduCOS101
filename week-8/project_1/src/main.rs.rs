use std::io;

// Reads one line from the keyboard and hands it back without the newline/spaces.
fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    println!("=== EY Nigeria Interview: Most Experienced Developer ===");

    // A vector of tuples: each item is (name, years of experience).
    // The vector grows as we push candidates in (Practice 5),
    // and each tuple mixes a String with a number (Part 2).
    let mut candidates: Vec<(String, u32)> = Vec::new();

    let how_many: usize = read_input("How many candidates are you interviewing?")
        .parse()
        .expect("Invalid input");

    for count in 0..how_many {
        println!("\nCandidate {}", count + 1);
        let name = read_input("Enter name:");
        let years: u32 = read_input("Enter years of programming experience:")
            .parse()
            .expect("Invalid input");

        candidates.push((name, years));
    }

    // Nobody was entered, so there is no winner. Stop here.
    if candidates.len() == 0 {
        println!("No candidates were entered.");
        return;
    }

    // Start by assuming the first candidate is the best.
    // We keep the POSITION of the best one so far.
    let mut best_index = 0;

    // Walk through everyone else. If someone beats the best so far, they become the new best.
    for i in 1..candidates.len() {
        // .1 reaches the years inside the tuple (Practice 7).
        // Strict ">" means that in a tie, the person interviewed first stays the winner.
        if candidates[i].1 > candidates[best_index].1 {
            best_index = i;
        }
    }

    // Unpack the winner's tuple into named variables (Practice 10).
    let (best_name, best_years) = &candidates[best_index];

    println!("\nThe most experienced candidate is {} with {} years.", best_name, best_years);
}



fn main() {
    // ths user inputs their profession
use std::io;
println!("What is your profession?");
println!("1. Office Admin");
println!("2. Academic");
println!("3. Lawyer");
println!("4. Teacher");
