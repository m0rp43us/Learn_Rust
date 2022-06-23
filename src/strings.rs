// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run(){
    let hello = "Hello";
    let mut another_hello = String::from("Hello World !");

    //Get length
    println!("Length of hello: {}",hello.len());
    println!("Length of another_hello: {}",another_hello.len());

    //push char
    another_hello.push('O');
    another_hello.push('h');
    another_hello.push(' ');

    //push string
    another_hello.push_str(hello);
    println!("{}",another_hello);
    println!("Capacity : {}",another_hello.capacity());
    println!("Is empty : {}",another_hello.is_empty());
    println!("contains World ? : {}",another_hello.contains("World"))
}