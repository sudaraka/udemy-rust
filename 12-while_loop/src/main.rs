fn main() {
  let mut x = 1;
  while 1000 > x {
    x *= 2;

    if 64 == x {
      continue;
    }

    println!("x = {}", x);
  }

  let mut y = 1;
  loop {
    y *= 2;
    println!("y = {}", y);

    if 1 << 10 == y {  // 1 << 10 means 2^10
      break;
    }
  }
}
