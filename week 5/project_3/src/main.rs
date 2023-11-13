use std::io;

fn main() {
    // Display menu
    println!("Menu\t\t\t\tPrice");
    println!("P = Poundo Yam/Edikaiko Soup\t- N3,200");
    println!("F = Fried Rice & Chicken\t- N3,000");
    println!("A = Amala & Ewedu Soup\t\t- N2,500");
    println!("E = Eba & Egusi Soup\t\t- N2,000");
    println!("W = White Rice & Stew\t\t- N2,500");

    // Input order details
    println!("Enter the type of food (P, F, A, E, W):");
    let mut food_type = String::new();
    io::stdin().read_line(&mut food_type).expect("Failed to read input");
    let food_type = food_type.trim();

    println!("Enter the quantity:");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let quantity: u32 = quantity.trim().parse().expect("Invalid input. Please enter a number.");

    // Calculate total charges based on food type and quantity
    let price = match food_type {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid food type.");
            return;
        }
    };

    let total_charges = price * quantity;

    // Apply discount if total order is greater than N10,000
    let discounted_total_charges = if total_charges > 10_000 {
        let discount = total_charges * 5 / 100; // 5% discount
        total_charges - discount
    } else {
        total_charges
    };

    // Display total charges
    println!("Total charges for the order: N{}", discounted_total_charges);
}
