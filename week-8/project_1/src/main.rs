
use std::io;

fn main() {
    // Vector containing the available professions
    let professions: Vec<&str> = vec![
        "Office Admin",
        "Academic",
        "Lawyer",
        "Teacher",
    ];

    // Vector containing job titles for each level.
    // Each inner vector follows the same profession order.
    let job_titles: Vec<Vec<&str>> = vec![
        vec!["Intern", "-", "Paralegal", "Placement"],
        vec![
            "Administrator",
            "Research Assistant",
            "Junior Associate",
            "Classroom Teacher",
        ],
        vec![
            "Senior Administrator",
            "PhD Candidate",
            "Associate",
            "Senior Teacher",
        ],
        vec![
            "Office Manager",
            "Post-Doc Researcher",
            "Senior Associate 1-2",
            "Leading Teacher",
        ],
        vec![
            "Director",
            "Senior Lecturer",
            "Senior Associate 3-4",
            "Deputy Principal",
        ],
        vec!["CEO", "Dean", "Partner", "Principal"],
    ];

    // Vector containing the level labels
    let levels: Vec<&str> = vec![
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
        "EL2 10-13",
        "SES",
    ];

    println!("=== FEDERAL PUBLIC SERVICE APS LEVEL CHECKER ===");
    println!("Available professions:");

    for i in 0..professions.len() {
        println!("{}. {}", i + 1, professions[i]);
    }

    println!("Enter your profession number:");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let choice: usize = match choice.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid input. Enter a number from 1 to 4.");
            return;
        }
    };

    if choice < 1 || choice > professions.len() {
        println!("Invalid profession number.");
        return;
    }

    let profession_index = choice - 1;

    println!("Enter your years of experience:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let years: u32 = match input.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid input. Enter a whole number.");
            return;
        }
    };

    // Determine the correct row in the table.
    // Assumption: 5 years belongs to APS 5-8.
    let level_index = if years >= 1 && years <= 2 {
        0
    } else if years >= 3 && years <= 4 {
        1
    } else if years >= 5 && years <= 8 {
        2
    } else if years >= 9 && years <= 10 {
        3
    } else if years >= 11 && years <= 13 {
        4
    } else if years >= 14 {
        5
    } else {
        println!("Experience must be at least 1 year.");
        return;
    };

    println!("\n=== STAFF LEVEL RESULT ===");
    println!("Profession: {}", professions[profession_index]);
    println!("Experience: {} years", years);
    println!("APS Level: {}", levels[level_index]);
    println!(
        "Job Title: {}",
        job_titles[level_index][profession_index]
    );
}
