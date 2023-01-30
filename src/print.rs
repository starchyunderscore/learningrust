pub fn run() {
    // print to console
    println!("Hello from the print file");
    // basic formatting
    println!("Number {}", 1);
    // positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");
    // named arguments
    println!("{name} likes to play {activity}", name = "John", activity = "baseball");
    // placeholder traits
    println!("Binary: {:b} Hex: {:x} octal: {:o}", 10, 10, 10);
    // placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));
    // basic math
    println!("10 + 10 = {}", 10+10);
}