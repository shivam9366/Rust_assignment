fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
  if k > arr.len() {
      return None;
  }

  // Create a min-heap
  let mut heap = std::collections::BinaryHeap::new();

  // Insert the first k elements into the heap
  for &num in arr.iter().take(k) {
      heap.push(num);
  }

  // For the remaining elements, compare with the root of the heap
  for &num in arr.iter().skip(k) {
      if let Some(&root) = heap.peek() {
          if num < root {
              heap.pop();
              heap.push(num);
          }
      }
  }

  heap.pop() // The kth smallest element is at the root of the heap
}

fn main() {
  let arr = vec![7, 10, 4, 3, 20, 15];
  let k = 3;

  if let Some(kth_smallest) = kth_smallest(&arr, k) {
      println!("The {}th smallest element is: {}", k, kth_smallest);
  } else {
      println!("Invalid input: k is out of range.");
  }
}
