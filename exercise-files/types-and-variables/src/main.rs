#![allow(dead_code)]
#![allow(unused_imports)]

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
}
