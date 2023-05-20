/* 
Primitive Types--
Integeres: u8, i18, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char) '1'
Tuples
Arrays
 */
/* Rust is a statically typed language, wich means that it must know the types of all variables at compile time, however, the compiler can usually infer what type
we want to use based on the value and how we use it 
 */
pub fn run (){

    //  default is "i32"
    let x = 1;

    // default is "f64"
    let y =2.5;

    // Add explicit type
    let z: i64 = 45435454545;   
 
    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX); 


    // Boolean
 
    let is_active = true;

    // Get boolean from expression ''

    let is_greater: bool = 10 < 5;

    let a1:char = '1'; 
    let face = '\u{1f600}';
    println!("{:?}", (x,y,z, is_active, is_greater, a1, face));

}