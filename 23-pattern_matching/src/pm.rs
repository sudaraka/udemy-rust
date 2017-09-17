pub fn pattern_matching() {
  for x in 0..13 {
    println!("{}: I have {} oranges", x, how_many(x));
  }

  let point = (3, 4);
  match point {
    (0, 0) => println!("origin"),
    (0, y) => println!("on x axis, y = {}", y),
    (ref x, 0) => println!("on y axis, x = {}", x),  // NOTE: in here `x` is passed by reference, `mut` is also possible.
    (x, y) => println!("x = {}, y = {}", x, y)
  }
}

fn how_many(x: i32) -> &'static str {
  match x {
    0 => "no",
    1 | 2 => "one or two",  // catch multiple cases/conditions/patterns
    12 => "a dozen",
    9...11 => "lots of",  // range pattern NOTE: last value is included
    _ if 0 == x % 2 => "some",  // conditional pattern
    _ => "a few"
  }
}
