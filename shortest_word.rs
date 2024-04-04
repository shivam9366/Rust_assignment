fn find_shortest_word(s: &str) -> Option<&str> {
  s.split_whitespace()
      .min_by_key(|word| word.len())
}

fn main() {
  let input = "I'm final year students of MLV Textile & Engineering College Bhilwara.";
  if let Some(shortest_word) = find_shortest_word(input) {
      println!("The shortest word in the string is: {}", shortest_word);
  } else {
      println!("No words found in the string.");
  }
}
