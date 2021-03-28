/*
 * QuickSort is a Divide and Conquer algorithm. It picks an element as pivot and partitions the given array around the picked pivot. There are many different versions of quickSort that pick pivot in different ways. 
 * - Always pick first element as pivot.
 * - Always pick last element as pivot (implemented below)
 * - Pick a random element as pivot.
 * - Pick median as pivot.
 */

fn partition<T: Ord>(array: &mut [T], low: isize, high: isize) -> isize {
  let pivot = high as usize;
  let mut index = low - 1;
  let mut last_index = high;

  loop {
    index += 1;

    while array[index as usize] < array[pivot] {
      index += 1;
    }

    last_index -= 1;

    while last_index >= 0 && array[last_index as usize] > array[pivot] {
      last_index -= 1;
    }

    if index >= last_index {
      break;
    } else {
      array.swap(index as usize, last_index as usize);
    }
  }

  array.swap(index as usize, pivot as usize);
  index
}

fn _quick_sort<T: Ord>(array: &mut [T], low: isize, high: isize) {
  if low >= high {
    return;
  }

  // pi => partitioning index
  let pi = partition(array, low as isize, high as isize);

  _quick_sort(array, low, pi - 1);
  _quick_sort(array, pi + 1, high);
}

pub fn quick_sort<T: Ord>(array: &mut [T]) {
  let len = array.len();

  _quick_sort(array, 0, (len - 1) as isize);
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
