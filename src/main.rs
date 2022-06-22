//mod func;
mod types;
fn main() {
    types::run();

    println!("number : {}",2);
    println!("{1} {0} {3} {2}","test1","test0","test3","test2");
    println!("{name} like to do {acitivity}",name = "jhon Doe",acitivity = "football");
    println!("binary {:b} hexadecimal {:x} octo : {:o}",24,24,24);
    //debug mode
    println!("{:?}",(10,true,"test"));
}
