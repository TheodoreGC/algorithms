/*
 * Merge Sort is a Divide and Conquer algorithm.
 * It divides the input array into two halves, calls itself for the two halves, and then merges the two sorted halves.
 * The merge() function is used for merging two halves.
 * The merge(arr, l, m, r) is a key process that assumes that arr[l..m] and arr[m+1..r] are sorted and merges the two sorted sub-arrays into one.
 * See the following Rust implementation for details.
 */

fn merge_sort<T: Copy + Ord>(x: &mut [T]) {
  // TODO
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
