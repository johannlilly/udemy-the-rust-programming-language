use std::mem;

fn main()
{
  // true false
  let g = false;
  println!("e = {}, size = {} bytes", g, mem::size_of_val(&g)); // 1 byte
  let f = 4>0; // true
}
