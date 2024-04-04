fn reverse_string(s: &str) -> String {
  s.chars().rev().collect()
}

fn main() {
  let input = "Shivam Chouhan";
  let reversed = reverse_string(input);
  println!("Original: {}", input);
  println!("Reversed: {}", reversed);
}
