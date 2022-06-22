/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.


pub fn run(){
//default is i32
let x = 1;

//default is f64
let y = 2.5;

//add explicit type
let z:i64 = 251263635;

//find max size
println!("Max i32 :{}",std::i32::MAX);
println!("Max i64 :{}",std::i64::MAX);
println!("Max f64 :{}",std::f64::MAX);

//Boolean
let is_greater = 10 > 5;
let boool = true;

//chars

let a1 = 'a';
let face = '\u{1F600}';

println!("{:?}",(x,y,z,boool,is_greater,a1,face));

}