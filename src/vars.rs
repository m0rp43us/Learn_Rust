// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
    let name = "Brad";
    let mut age = 17;
    println!("my name is {} and i got {} y.o",name,age);
    age = 39;
    println!("my name is {} and i got {} y.o",name,age);

    //define a constant
    const ID:i32 = 001;
    println!("ID : {}",ID);

    let (my_name ,my_age) = ("Brad",37);
    println!("{} is {}",my_name,my_age)
}