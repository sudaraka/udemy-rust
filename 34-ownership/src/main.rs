fn main() {
  let v = vec![ 1, 2, 3 ];  // v owns the heap memory for the vector

  // Example #1
  // let v2 = v;  // v2 does NOT own vector memory
  // println!("{:?}", v);  // Rust take away the ownership from v and assign it to v2

  // Example #2
  // let foo = |v: Vec<i32>| ();
  // foo(v);
  // println!("{:?}", v);  // Rust take away the ownership from v in outer scope and assign it to v in closure

  // Example #3
  let u = 1;  // i32
  let u2 = u;
  println!("u = {}", u);  // Rust does not "move" (reassign ownership for) primitives

  // Example #4
  // let u = Box::new(1);  // Primitive value in a "Box", putting the value in heap.
  // let u2 = u;
  // println!("u = {}", *u);  // Rust take away the ownership from u and assign it to u2

  // EXample #5
  let print_vector = |x:Vec<i32>| -> Vec<i32> {
    println!("{:?}", x);

    x  // return vector
  };
  let v3 = print_vector(v);  // `print_vector` takes the ownership of `v`, but give it back by returning the same vector.
  println!("{:?}", v3[0]);
}
