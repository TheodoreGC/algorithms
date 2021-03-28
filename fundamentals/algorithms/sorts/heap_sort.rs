/*
 * Heap sort is a comparison based sorting technique based on Binary Heap data structure.
 * It is similar to selection sort where we first find the maximum element and place the maximum element at the end.
 * We repeat the same process for the remaining elements.
 */

fn move_down<T: Ord>(array: &mut [T], mut root: usize) {
  let last = array.len() - 1;

  loop {
    let left = 2 * root + 1;

    if left > last {
      break;
    }

    let right = left + 1;
    let max = if right <= last && array[right] > array[left] {
      right
    } else {
      left
    };

    if array[max] > array[root] {
      array.swap(root, max);
    }

    root = max;
  }
}

fn heapify<T: Ord>(array: &mut [T]) {
  let last_parent = (array.len() - 2) / 2;

  for i in (0..=last_parent).rev() {
    move_down(array, i);
  }
}

fn heap_sort<T: Ord>(array: &mut [T]) {
  if array.len() <= 1 {
    return;
  }

  heapify(array);

  for end in (1..array.len()).rev() {
    array.swap(0, end);
    move_down(&mut array[..end], 0);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty() {
    let mut array: Vec<i32> = Vec::new();
    heap_sort(&mut array);
    assert_eq!(&array, &[]);
  }

  #[test]
  fn test_single_element() {
      let mut arr = vec![1];
      heap_sort(&mut arr);
      assert_eq!(&arr, &[1]);
  }

  #[test]
  fn test_sorted_array() {
      let mut arr = vec![1, 2, 3, 4];
      heap_sort(&mut arr);
      assert_eq!(&arr, &[1, 2, 3, 4]);
  }

  #[test]
  fn test_unsorted_array() {
      let mut arr = vec![3, 4, 2, 1];
      heap_sort(&mut arr);
      assert_eq!(&arr, &[1, 2, 3, 4]);
  }

  #[test]
  fn test_odd_number_of_elements() {
      let mut arr = vec![3, 4, 2, 1, 7];
      heap_sort(&mut arr);
      assert_eq!(&arr, &[1, 2, 3, 4, 7]);
  }

  #[test]
  fn test_repeated_elements() {
      let mut arr = vec![542, 542, 542, 542];
      heap_sort(&mut arr);
      assert_eq!(&arr, &vec![542, 542, 542, 542]);
  }

  #[test]
  fn test_ascending_number_sorting() {
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    heap_sort(&mut numbers);
    assert_eq!(numbers, [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]);
  }

  #[test]
  fn test_alphabetical_strings_sorting() {
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    heap_sort(&mut strings);
    assert_eq!(strings, ["airplane", "art", "beach", "car", "hotel", "house"]);
  }
}
