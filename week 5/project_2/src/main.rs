use std::io;

fn main() {
    // Input employee details
    println!("Enter the experience (experienced or inexperienced):");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim();

    println!("Enter the age of the employee:");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age: u32 = age.trim().parse().expect("Invalid input. Please enter a number.");

    // Determine annual incentive based on criteria
    let incentive = match (experience, age) {
        ("experienced", age) if age >= 40 => 1_560_000,
        ("experienced", age) if age >= 30 && age < 40 => 1_480_000,
        ("experienced", age) if age < 28 => 1_300_000,
        ("inexperienced", _) => 100_000,
        _ => {
            println!("Invalid input or criteria not met.");
            return;
        }
    };

    // Display the annual incentive
    println!("The annual incentive for the employee is: N{}", incentive);
}

