use std::mem;

pub fn run() {
    // create array, size and data type must be present and accurate
    let mut numbers: Vec<i32> = vec![1,2,3,4];
    
    // reassign value
    numbers[2] = 20;
    
    // print whole vector
    println!("{:?}", numbers);
    
    // print single item
    println!("Single value {}", numbers[0]);
    
    // get vector length
    println!("Array length: {}", numbers.len());
    
    // vectors are stack allocated // if no use std::mem; at the top, std::mem::size_of_val(&numbers));
    println!("Vectors occupies {} bytes", mem::size_of_val(&numbers));
    
    // get slice // [inclusive..exclusive]
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}