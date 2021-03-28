/*
 * Merge Sort is a Divide and Conquer algorithm.
 * It divides the input array into two halves, calls itself for the two halves, and then merges the two sorted halves.
 * The merge() function is used for merging two halves.
 * The merge(arr, l, m, r) is a key process that assumes that arr[l..m] and arr[m+1..r] are sorted and merges the two sorted sub-arrays into one.
 * See the following Rust implementation for details.
 */

fn merge<T: Copy + PartialOrd>(array1: &[T], array2: &[T], vector: &mut [T]) {
  assert_eq!(array1.len() + array2.len(), vector.len());
  let mut i = 0;
  let mut j = 0;
  let mut k = 0;

  while i < array1.len() && j < array2.len() {
    if array1[i] < array2[j] {
      vector[k] = array1[i];
      k += 1;
      i += 1;
    } else {
      vector[k] = array2[j];
      k += 1;
      j += 1;
    }
  }

  if i < array1.len() {
    vector[k..].copy_from_slice(&array1[i..]);
  }

  if j < array1.len() {
    vector[k..].copy_from_slice(&array2[j..]);
  }
}

fn merge_sort<T: Copy + Ord>(array: &mut [T]) {
  let n = array.len();
  let m = n / 2;

  if n <= 1 {
    return;
  }

  merge_sort(&mut array[0..m]);
  merge_sort(&mut array[m..n]);

  let mut vector: Vec<T> = array.to_vec();

  merge(&array[0..m], &array[m..n], &mut vector[..]);

  array.copy_from_slice(&vector);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ascending_number_sorting() {
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    merge_sort(&mut numbers);
    assert_eq!(numbers, [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]);
  }

  #[test]
  fn test_alphabetical_strings_sorting() {
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    merge_sort(&mut strings);
    assert_eq!(strings, ["airplane", "art", "beach", "car", "hotel", "house"]);
  }
}
