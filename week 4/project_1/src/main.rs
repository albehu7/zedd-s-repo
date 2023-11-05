fn main() {
    // Given distance in miles and time in hours
    let distance1_miles = 80.0;
    let time1_hours = 2.0;

    let distance2_miles = 120.0;
    let time2_hours = 4.0;

    // Conversion factor from miles to kilometers
    let miles_to_km = 1.60934;

    // Calculate the speed for the first case
    let speed1_kph = (distance1_miles * miles_to_km) / time1_hours;

    // Calculate the speed for the second case
    let speed2_kph = (distance2_miles * miles_to_km) / time2_hours;

    // Print the results
    println!("Speed for the first case: {:.2} km/h", speed1_kph);
    println!("Speed for the second case: {:.2} km/h", speed2_kph);
}


