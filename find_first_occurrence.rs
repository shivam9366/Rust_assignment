fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
  // Binary search to find the first occurrence
  let mut left = 0;
  let mut right = arr.len();

  while left < right {
      let mid = left + (right - left) / 2;
      if arr[mid] < target {
          left = mid + 1;
      } else {
          right = mid;
      }
  }

  if left < arr.len() && arr[left] == target {
      Some(left)
  } else {
      None
  }
}

fn main() {
  let arr = vec![1, 2, 3, 3, 4, 5, 5, 6, 7];
  let target = 3;

  if let Some(index) = find_first_occurrence(&arr, target) {
      println!("The first occurrence of {} is at index {}", target, index);
  } else {
      println!("{} not found in the array", target);
  }
}
