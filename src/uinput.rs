pub fn run () {
    println!("You wrote: {}", read_input().trim());
}

fn read_input() -> String {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
} 