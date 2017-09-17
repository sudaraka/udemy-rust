fn main() {
  let sh = say_hello;
  sh();

  let plus_one = |x: i32| -> i32 {  // plus_one is a closure of main
    x + 1
  };
  let a = 6;
  println!("{} + 1 = {}", a, plus_one(a));

  let mut two = 2;
  {  // start new scope
    let plus_two = |x| {  // plus_two is a closure of new scope
      let mut z = x;

      z += two;

      z
    };
    let b = 8;
    println!("{} + 2 = {}", b, plus_two(b));
  }

  let borrow_two = &mut two;

  // T: by value
  let plus_three = |x: &mut i32| *x += 3;
  let mut f = 12;
  plus_three(&mut f);
  println!("f = {}", f);
}

fn say_hello() {
  println!("hello");
}
