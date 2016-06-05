use std::mem;
fn main() {
    let a:u8 = 123;
    println!("the value of a is {}", a);
    let mut b:i8 = 0;
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);
    let mut c = 123456789;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}", c);
    let z:isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);
}