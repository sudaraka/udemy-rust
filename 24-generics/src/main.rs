struct Point<T> {
  x: T,
  y: T
}

struct Line<T> {
  start: Point<T>,
  end: Point<T>
}

fn main() {
  let a = Point { x: 0.0, y: 4f64 };  // Point<f64>
  let b = Point { x: 1.2, y: 3.4 };  // Point<f64>
  let l = Line { start: a, end: b };
}
