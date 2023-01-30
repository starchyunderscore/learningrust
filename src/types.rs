pub fn run() {
    // default i32
    let x = 1;
    // default f64
    let y = 2.5;
    // explicit type
    let z :i64 = 4545445454545;
    println!("max i32: {}", std::i32::MAX);
    println!("max i64: {}", std::i64::MAX);
    // boolean
    let is_active = true;
    let is_greater = 10 < 5;
    // char
    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}