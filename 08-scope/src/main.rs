fn main() {
  let a = 123;
  let a = 456;

  {
    let b = 156;
    println!("inside, b = {}", b);

    let a = 777;
    println!("inside, a = {}", a);
  }

  println!("a = {}", a);
}
