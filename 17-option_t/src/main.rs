fn main() {
  let x = 3.0;
  let y = 2.0;

  let result =
    if 0.0 != y {
      Some(x / y)
    }
    else {
      None
    };

  match result {
    Some(z) => println!("{} ÷ {} = {}", x, y, z),

    None => println!("Cannot divide {} by {}", x, y)
  }


  if let Some(z) = result {
    println!("{} ÷ {} = {}", x, y, z);
  }
}
