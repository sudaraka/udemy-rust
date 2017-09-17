enum Color {
  Red,
  Green,
  Blue,
  RgbColor (u8, u8, u8),
  CymkColor { cyan: u8, magenta: u8, yellow: u8, black: u8 }
}

fn main() {
  let c = Color::CymkColor {
    cyan: 0,
    magenta: 128,
    yellow: 0,
    black: 255
  };

  match c {
    Color::Red => println!("r"),
    Color::Blue => println!("b"),
    Color::Green => println!("g"),
    Color::RgbColor(0, 0, 0)
      | Color::CymkColor {
        cyan: _,
        magenta: _,
        yellow: _,
        black: 255 }
      | Color::CymkColor {
        black: 255,
        ..} => println!("black"),
    Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
    _ => ()
  }
}
