pub fn run() {
    let name = "Brad";
    let mut age = 37;
    println!("{}",age);
    age = 38;
    println!("My name is {} and i am {}", name, age);
    // define constant
    const ID :i32 = 001;
    println!("ID: {}", ID);
    // assign multiple variables
    let ( my_name, my_age ) = ("brad", 37);
    println!("{}, {}", my_name, my_age);
}