fn main() {
  let mut data = [ 1, 2, 3, 4, 5 ];

  useSlice(&mut data[1..4]);

  println!("{:?}", data);
}

fn useSlice(slice: &mut [i32]) {
  println!("first element = {}, len = {}", slice[0], slice.len());

  slice[0] = 123;
}
