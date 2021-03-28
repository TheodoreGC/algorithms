/*
 * Breadth-first search (BFS) is an algorithm for traversing or searching tree or graph data structures.
 * It starts at the tree root (or some arbitrary node of a graph, sometimes referred to as a 'search key'[1]), and explores all of the neighbor nodes at the present depth prior to moving on to the nodes at the next depth level.
 * It uses the opposite strategy of depth-first search, which instead explores the node branch as far as possible before being forced to backtrack and expand other nodes.
 */

fn bfs<T: PartialEq + PartialOrd>(item: &T, array: &[T]) -> i32 {
  // TODO
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bfs() {
    let mut strings = vec!["beach", "hotel", "airplane", "car", "house", "art"];
    let index = bfs(&"car", &strings);
    assert_eq!(index, 3);
  }
}
