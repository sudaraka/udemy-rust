use std::mem;

struct Point {
  x: f64,
  y: f64
}

fn origin() -> Point {
  Point{ x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
  let p1 = origin();
  let p2 = Box::new(origin());

  println!("p1 takes up {} bytes", mem::size_of_val(&p1));  // 16 bytes (128 bits or 2 x 64-bit x and y) on stack
  println!("p2 takes up {} bytes", mem::size_of_val(&p2));  // 8 bytes (64-but (usize, OS specific address size) on stack, which is pointing to p1 like structure on heap)
}
