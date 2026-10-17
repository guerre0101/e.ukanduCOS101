
use std::io;

fn main() {
    // A vector that will store each candidate's name and experience.
    // Each tuple contains (name, years of experience).
    let mut candidates: Vec<(String, u32)> = Vec::new();

    println!("=== EY NIGERIA PROGRAMMING INTERVIEW ===");
    println!("How many candidates attended the interview?");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number_of_candidates: usize = match input.trim().parse() {
        Ok(number) if number > 0 => number,
        _ => {
            println!("Please enter a valid number greater than zero.");
            return;
        }
    };

    // Collect each candidate's information.
    for i in 0..number_of_candidates {
        println!("\nEnter the name of candidate {}:", i + 1);

        let mut name = String::new();

        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read candidate name");

        let name = name.trim().to_string();

        if name.is_empty() {
            println!("Candidate name cannot be empty.");
            return;
        }

        println!("Enter {}'s years of programming experience:", name);

        let mut experience_input = String::new();

        io::stdin()
            .read_line(&mut experience_input)
            .expect("Failed to read experience");

        let years: u32 = match experience_input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a valid whole number.");
                return;
            }
        };

        // Store the name and experience together as a tuple.
        candidates.push((name, years));
    }

    // Start by assuming the first candidate has the highest experience.
    let mut highest_index = 0;

    // Compare the remaining candidates with the current highest.
    for i in 1..candidates.len() {
        if candidates[i].1 > candidates[highest_index].1 {
            highest_index = i;
        }
    }

    // Retrieve the most experienced candidate.
    let winner = &candidates[highest_index];

    println!("\n=== EY INTERVIEW RESULT ===");
    println!("Most experienced candidate: {}", winner.0);
    println!("Years of programming experience: {}", winner.1);
}
