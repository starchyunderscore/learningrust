pub fn run() {
    // primitive string
    let hello = "Hello";
    
    // complex string
    let mut hello2 = String::from("Hello ");
    
    // get length
    println!("Length: {}", hello2.len());
    
    // push char
    hello2.push('W');
    
    // push string
    hello2.push_str("orld!");
    
    // capacity in bytes
    println!("Capacity: {}", hello2.capacity());
    
    // is empty
    println!("is empty: {}", hello2.is_empty());
    
    // contains substring
    println!("contains 'World': {}", hello2.contains("World"));
    
    // replace
    println!("replace: {}", hello2.replace("World", "There"));
    
    // loop through string by whitispace
    for word in hello2.split_whitespace() {
        println!("{}", word);
    }
    
    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}",s);
    
    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
        
    // prints
    println!("{} /// {}", hello, hello2);
}