use std::mem;

fn main() {
  // unsigned 0..255
  let a: u8 = 123;  //8-bit unsigned integer
  println!("a = {}", a);

  // a = 456;  // ERROR! a is not mutable

  // mut
  let mut b: u8 = 0;
  println!("b = {}", b);

  b = 42;
  println!("b = {}", b);

  // type infrence
  let mut c = 123456789;  // 32-bit signed integer
  println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

  c = -1;
  println!("c = {} after modification", c);


  let z: isize = 123;
  let size_of_z = mem::size_of_val(&z);
  println!("z = {}, takes up {} bytes, {}-bit OS",
           z, size_of_z, size_of_z * 8);

  let d = 'x';
  println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

  let e = 2.5;  // double-precision, 8 byte or 64-bit, f64
  println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

  let f: f32 = 2.5;  // double-precision, 4 byte or 32-bit, f32
  println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));

  // boolean
  let g = false;
  println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}
