/*
 * QuickSort is a Divide and Conquer algorithm. It picks an element as pivot and partitions the given array around the picked pivot. There are many different versions of quickSort that pick pivot in different ways. 
 * - Always pick first element as pivot.
 * - Always pick last element as pivot (implemented below)
 * - Pick a random element as pivot.
 * - Pick median as pivot.
 */

fn quick_sort<T: Ord>(array: &mut [T]) {
  // TODO
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ascending_number_sorting() {
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    quick_sort(&mut numbers);
    assert_eq!(numbers, [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]);
  }

  #[test]
  fn test_alphabetical_strings_sorting() {
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    quick_sort(&mut strings);
    assert_eq!(strings, ["airplane", "art", "beach", "car", "hotel", "house"]);
  }
}
