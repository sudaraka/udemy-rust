fn main() {
  // arithmetic
  let mut a = 2 + 3 * 4;
  println!("a = {}", a);

  a = a + 1;  // No ++ or --
  println!("a = {}", a);

  a -= 2;  // a = a - 2;
  println!("a = {}", a);

  println!("remainder of {} / {} = {}", a, 3, (a % 3));

  let a_cubed = i32::pow(a, 3);
  println!("{} cubed is {}", a, a_cubed);

  let b = 2.5;
  let b_cubed = f64::powi(b, 3);
  println!("{} cubed is {}", b, b_cubed);

  let b_to_pi = f64::powf(b, std::f64::consts::PI);
  println!("{}^pi is {}", b, b_to_pi);


  // bitwise
  let c = 1 | 2;
  println!("1 | 2 = {}", c);

  let two_to_10 = 2 << 10;
  println!("2 ^ 10 = {}", two_to_10);

  // logical
  let pi_less_4 = std::f64:consts::PI < 4.0;  // true
  let x_is_5 = x == 5;  // true
}
