fn main() {
  let temp = 35;

  if 30 < temp {
    println!("really hot outside");
  }
  else if 10 > temp {
    println!("really cold!");
  }
  else {
    println!("temperature is OK");
  }

  let day = if 20 < temp { "sunny" } else { "cloudy" };
  println!("today is {}", day);

  println!("it is {}",
           if 20 < temp { "hot" } else if 10 > temp { "cold" } else { "OK" }
           );

  println!("it is {}",
           if 20 < temp {
             if 30 < temp { "very hot" } else { "hot" }
           } else if 10 > temp { "cold" } else { "OK" }
           );
}
