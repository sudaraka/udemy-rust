fn main() {
  let country_code = 9990;  // valid codes are 1 to 999

  let country = match country_code {
    1 => "US",
    44 => "UK",
    46 => "Sweden",
    7 => "Russia",
    94 => "Sri Lanka",
    1...999 => "Unknown",  // NOTE: 3 dots, upper-bound is included
    _ => "Invalid"
  };

  println!("the country with code {} is {}", country_code, country);
}
