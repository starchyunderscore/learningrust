pub fn run() {
    println!("{}",double(8));
    println!("{}", complex(5));
}

fn double(input:i32) -> i32 {
    input * 2
}

fn complex(input:i32) -> i32 {
    let doubled = double(input);
    let halved = input/2;
    doubled - halved
}