/*
 * Bubble sorting is the simplest sorting algorithm.
 * It simply iterates over the entire list and compare adjacent elements in the list, and after each comparison, place them in the right order of magnitude.
 * This works by swapping adjacent items if they are not in the correct order.
 * The process is repeated n-1 times for a list of n items.
 * In each such iteration, the largest element is arranged in the end.
 */

fn bubble_sort<T: Ord>(array: &mut [T]) {
  // TODO
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ascending_number_sorting() {
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    bubble_sort(&mut numbers);
    assert_eq!(numbers, [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]);
  }

  fn test_alphabetical_strings_sorting() {
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    bubble_sort(&mut strings);
    assert_eq!(strings, ["airplane", "art", "beach", "car", "hotel", "house"]);
  }
}
