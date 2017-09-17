fn main() {
  let v = vec![ 3, 2, 1 ];  // v owns the heap memory for the vector

  let print_vector = |x: &Vec<i32>| {
    println!("x[0] = {}", x[0]);
  };
  print_vector(&v);  // `print_vector` take a reference to the vector, therefore borrowing it and not taking ownership.
  println!("v[0] = {}", v[0]);

  // let mut a = 40;
  // let b = &mut a;
  // *b += 2;
  // println!("a = {}", a);  // `a` has been borrowed by `b` as `a` mutable reference

  let mut a = 40;
  {
    let b = &mut a;
    *b += 2;
  }
  // `a` has been borrowed by `b` as `a` mutable reference. but `b` goes out of
  // scope before we use `a` again, return borrowed reference.
  println!("a = {}", a);

  let mut z = vec![ 3, 2, 1 ];  // `z` owns the heap memory for the vector
  for i in &z {  // `i` is an immutable reference to element in `z`
    println!("i = {}", i);

    z.push(5);
  }
}
