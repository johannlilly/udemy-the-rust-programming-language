use std::mem;

fn main()
{
  // arithmetic

  let mut a = 2+3*4; // +-*/
  println!("{}", a);
  a = a+1; // no increment or decrement
  a -= 2; // valid

  println!("remainder of {} / {} = {}", a, 3, (a%3));
  // modulo = remainder of
  // no power operator

  let a_cubed = i32::pow(a, 3);
  println!("{} cubed is {}", a, a_cubed);
}
