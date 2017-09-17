#[cfg(test)]
mod tests {
  extern crate own_crate;

  #[test]
  // #[ignore]
  // #[should_panic]
  fn english_greeting_correct() {
    assert_eq!("hello", own_crate::greetings::english::hello())
  }
}
