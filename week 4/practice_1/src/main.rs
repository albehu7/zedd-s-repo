fn main() {
    let name = "Aisha lawal";
    let uni: &str = "Pan Atlantic University";
    let address: &str = "Km 52 lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {}",name );
    println!("University: {}", address);


    let department:&'static str = "Computer Science";
    let school: &'static str = "School Of Science and Technology";
    println!("Department :{},\nSchool: {}", department, school);
}
