fn longest_common_prefix(arr: &[&str]) -> String {
  if arr.is_empty() {
      return String::new();
  }
  let mut result = arr[0].to_string();
  for i in 1..arr.len() {
      while !arr[i].starts_with(&result) {
          result.pop();
      }
      if result.is_empty() {
          return String::from("-1"); // No common prefix found
      }
  }
  result
}

fn main() {
  let input = ["geeksforgeeks", "geeks", "geek", "geezer","geeing"];
  let common_prefix = longest_common_prefix(&input);
  println!("The longest common prefix is: {}", common_prefix);
}
