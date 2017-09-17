fn main() {
  let mut a = Vec::new();

  a.push(1);
  a.push(2);
  a.push(3);

  println!("{:?}", a);

  a.push(44);
  println!("{:?}", a);

  println!("a[0] = {}", a[0]);

  let idx:usize = 10;
  // NOTE: following 2 statements cause index out of bound panic
  // a[idx] = 321;
  // println!("a[0] = {}", a[idx]);

  // Proper way to handle out of bound indexes
  match a.get(idx) {
    None => println!("error: no element at {}", idx),

    Some(x) => println!("a[{}] = {}", idx, x)
  }

  for x in &a {
    println!("{}", x);
  }

  a.push(66);
  println!("{:?}", a);

  let last = a.pop();
  println!("last element is {:?}, a = {:?}", last, a);

  while let Some(x) = a.pop() {
    println!("{}", x);
  }
}
