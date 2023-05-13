pub fn run () {
    println!("Write somethnig: ");
    println!("You wrote: {}", read_input().trim());
    if continu() {
        println!("e");
    }
}

fn read_input() -> String {
    let mut input:String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn continu() -> bool {
    println!("continue? (y/n)");
    let mut input:String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim() == "y"
}