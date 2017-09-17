extern crate rand;
extern crate own_crate;

use rand::Rng;
use own_crate::greetings::french;

fn main() {
  // let mut rng = rand::thread_rng();
  // let b: bool = rng.gen();
  //
  // println!("{:?}", b);

  println!("English: {}, {}",
           own_crate::greetings::english::hello(),
           own_crate::greetings::english::goodbye(),
           );

  println!("French: {}, {}",
           french::hello(),
           french::goodbye(),
           );
}
