/*
 * Depth-first search (DFS) is an algorithm for traversing or searching tree or graph data structures.
 * The algorithm starts at the root node (selecting some arbitrary node as the root node in the case of a graph) and explores as far as possible along each branch before backtracking.
 */

fn dfs<T: PartialEq + PartialOrd>(item: &T, array: &[T]) -> i32 {
  // TODO
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_dfs() {
    let mut strings = vec!["beach", "hotel", "airplane", "car", "house", "art"];
    let index = dfs(&"car", &strings);
    assert_eq!(index, 3);
  }
}
