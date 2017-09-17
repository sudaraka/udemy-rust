fn main() {
  for x in 1..11 {  // upper-bound is not inclusive
    if 3 == x {
      continue;
    }

    if 8 == x {
      break;
    }

    println!("x = {}", x);
  }

  for (pos, y) in (30..41).enumerate() {  // upper-bound is not inclusive
    println!("{}:  {}", pos, y);
  }
}
