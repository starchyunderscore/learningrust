pub fn run() {
    println!("{}",double(8));
    println!("{}", complex(5));
    let a_number:i32=8;
    println!("{} is even? {}",a_number,is_even(a_number))
}

fn double(input:i32) -> i32 {
    input * 2
}

fn complex(input:i32) -> i32 {
    let doubled = double(input);
    let halved = input/2;
    doubled - halved
}

fn is_even(num:i32) -> bool {
    num%2==0
}