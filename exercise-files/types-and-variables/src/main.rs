use std::mem;

fn main()
{
  let d = 'x';
  println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));
  
}
