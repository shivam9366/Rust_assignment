fn find_median(arr: &[i32]) -> f64 {
  let len = arr.len();
  if len % 2 == 0 {
      // Even number of elements
      let left_index = len / 2 - 1;
      let right_index = len / 2;
      (arr[left_index] + arr[right_index]) as f64 / 2.0
  } else {
      // Odd number of elements
      arr[len / 2] as f64
  }
}

fn main() {
  let sorted_array = vec![1, 2, 3, 4, 5, 6, 7, 8];
  let median = find_median(&sorted_array);
  println!("The median of the array is: {}", median);
}

