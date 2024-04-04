fn max_subarray_sum(arr: &[i32]) -> i32 {
  let mut max_sum = arr[0];
  let mut current_sum = arr[0];

  for &num in arr.iter().skip(1) {
      current_sum = current_sum.max(num);
      max_sum = max_sum.max(current_sum);
  }

  max_sum
}

fn main() {
  let input = vec![-2, -3, 4, -1, -2, 1, 5, -3];
  let result = max_subarray_sum(&input);
  println!("Maximum subarray sum: {}", result);
}
