use std::io;

fn main() {
    // Input coefficients
    println!("Enter the values of coefficients (a, b, c) for the quadratic equation:");

    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).expect("Failed to read input");
    let a: f64 = input_a.trim().parse().expect("Invalid input. Please enter a number.");

    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).expect("Failed to read input");
    let b: f64 = input_b.trim().parse().expect("Invalid input. Please enter a number.");

    let mut input_c = String::new();
    io::stdin().read_line(&mut input_c).expect("Failed to read input");
    let c: f64 = input_c.trim().parse().expect("Invalid input. Please enter a number.");

    // Calculate discriminant
    let discriminant = b.powi(2) - 4.0 * a * c;

    // Determine roots based on discriminant
    if discriminant > 0.0 {
        // Two distinct real roots
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The roots are real and distinct: Root1 = {:.2}, Root2 = {:.2}", root1, root2);
    } else if discriminant == 0.0 {
        // Exactly one real root
        let root = -b / (2.0 * a);
        println!("The root is real and equal: Root = {:.2}", root);
    } else {
        // No real roots (complex roots)
        let real_part = -b / (2.0 * a);
        let imaginary_part = (discriminant.abs().sqrt()) / (2.0 * a);
        println!(
            "The roots are complex: Root1 = {:.2} + {:.2}i, Root2 = {:.2} - {:.2}i",
            real_part,
            imaginary_part,
            real_part,
            imaginary_part
        );
    }
}


