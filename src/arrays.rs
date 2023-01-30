use std::mem;

pub fn run() {
    // create array, size and data type must be present and accurate
    let mut numbers: [i32; 4] = [1,2,3,4];
    
    // reassign value
    numbers[2] = 20;
    
    // print whole array
    println!("{:?}", numbers);
    
    // print single item
    println!("Single value {}", numbers[0]);
    
    // get array length
    println!("Array length: {}", numbers.len());
    
    // arrays are stack allocated // if no use std::mem; at the top, std::mem::size_of_val(&numbers));
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));
    
    // get slice // [inclusive..exclusive]
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}