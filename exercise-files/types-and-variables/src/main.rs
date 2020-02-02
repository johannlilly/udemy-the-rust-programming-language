use std::mem;

fn main()
{
  let a:u8 = 123; // 8bits, unsigned (0...255), mutable by default
  let _a:i8 = -123; // 8bits, signed (-128...127), mutable
  println!("a = {}", a); // variable is injected into curly braces

  // mut = mutable keyword
  let mut b:i8 = 0; // mutable
  println!("b = {}", b);
  b = 42;
  println!("b = {}", b);

  let mut c = 123456789; // let rust guess that this is 32-bit signed, aka i32
  println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
  c = -1;
  println!("c = {} after modification", c);

  // additional types: i8 u8 i16 u16 i32 u32 i64 u64
  let z:isize = 123;
  let size_of_z = mem::size_of_val(&z);
  // isize/usize are integral data types corresponding to the size of the pointer if you had an address in memory. In 64-bit systems you would have a 64-bit variable.
  println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);
  
}
