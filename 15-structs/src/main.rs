struct Point {
  x: f64,
  y: f64
}

struct Line {
  start: Point,
  end: Point
}

fn main() {
  let p = Point { x: 3.0, y: 4.0 };
  println!("point p is at ({}, {})", p.x, p.y);

  let p2 = Point { x: 5.0, y: 10.0 };
  let l = Line { start: p, end: p2 };
  println!("line l start at ({}, {}), and end at ({}, {})",
           l.start.x, l.start.y,
           l.end.x, l.end.y
           );
}
