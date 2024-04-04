fn is_prime(n: u32) -> bool {
  if n <= 1 {
      return false;
  }
  for a in 2..=(n as f64).sqrt() as u32 {
      if n % a == 0 {
          return false;
      }
  }
  true
}

fn main() {
  let num = 24; // Change the number here
  if is_prime(num) {
      println!("{} is a prime number.", num);
  } else {
      println!("{} is not a prime number.", num);
  }
}
