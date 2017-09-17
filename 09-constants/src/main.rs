
const GLOBAL_VALUE:u8 = 42;  // no fixed address, compiler replace occurrences with the value of a const

static Z:i32 = 123;
static mut X:i32 = 456;

fn main() {
    println!("{}", GLOBAL_VALUE);
    println!("{}", Z);

    unsafe {
      X = 777;

      println!("{}", X);
    }
}
