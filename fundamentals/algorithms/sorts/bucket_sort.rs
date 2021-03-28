/*
 * Bucket sort is mainly useful when input is uniformly distributed over a range. For example, consider the following problem. 
 * Sort a large set of floating point numbers which are in range from 0.0 to 1.0 and are uniformly distributed across the range. How do we sort the numbers efficiently?
 * A simple way is to apply a comparison based sorting algorithm. The lower bound for Comparison based sorting algorithm (Merge Sort, Heap Sort, Quick-Sort .. etc) is Ω(n Log n), i.e., they cannot do better than nLogn.
 * Can we sort the array in linear time? Counting sort can not be applied here as we use keys as index in counting sort. Here keys are floating point numbers.
 * The idea is to use bucket sort.
 */

fn bucket_sort<T: Ord>(array: &mut [T]) {
  // TODO
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ascending_number_sorting() {
    let mut numbers = [0.897, 0.565, 0.656, 0.1234, 0.665, 0.3434];
    bubble_sort(&mut numbers);
    assert_eq!(numbers, [0.1234, 0.3434, 0.565, 0.656, 0.665, 0.897]);
  }
}
