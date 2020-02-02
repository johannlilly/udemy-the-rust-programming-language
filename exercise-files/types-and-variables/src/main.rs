use std::mem;

fn main()
{
  // non-whole numbers
  let e = 2.5; // double-precision value, 8 bytes or 64 bits, f64
  //  e:f32 = 2.5
  println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));
}
