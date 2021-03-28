/*
 * Search a sorted array by repeatedly dividing the search interval in half.
 * Begin with an interval covering the whole array.
 * If the value of the search key is less than the item in the middle of the interval, narrow the interval to the lower half.
 * Otherwise narrow it to the upper half.
 * Repeatedly check until the value is found or the interval is empty.
 */

fn binary_search<T: PartialEq + PartialOrd>(item: &T, array: &[T]) -> i32 {
  // TODO
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_binary_search() {
    let mut strings = vec!["beach", "hotel", "airplane", "car", "house", "art"];
    let index = binary_search(&"car", &strings);
    assert_eq!(index, 3);
  }
}
