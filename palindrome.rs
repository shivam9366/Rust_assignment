fn is_palindrome(s: &str) -> bool {
  let s = s.to_lowercase(); // Convert the string to lowercase for case-insensitive comparison
  let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>(); // Remove non-alphanumeric characters
  let reversed = s.chars().rev().collect::<String>(); // Reverse the string
  s == reversed // Compare both strings
}

fn main() {
  let input = "MEM"; // Change the input string here
  if is_palindrome(input) {
      println!("\"{}\" is a palindrome!", input);
  } else {
      println!("\"{}\" is not a palindrome.", input);
  }
}
