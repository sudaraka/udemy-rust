fn main() {
  // str / string slice
  let s = "hello there!";

  // NOT allowed for a string slice
  // s = "abc";
  // let c = s[0];

  for c in s.chars().rev() {
    println!("{}", c);
  }

  if let Some(firstChar) = s.chars().nth(0) {
    println!("first letter is {}", firstChar);
  }

  // String
  let mut letters = String::new();
  let mut a = 'a' as u8;

  while a <= ('z' as u8) {
    letters.push(a as char);
    letters.push_str(",");

    a += 1;
  }

  println!("{}", letters);
}
