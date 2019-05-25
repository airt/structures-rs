/// Max Heap
///
/// # Examples
///
/// ```
/// use structures::heap::Heap;
///
/// let mut heap = Heap::new();
///
/// heap.push(2);
/// heap.push(1);
/// heap.push(3);
///
/// assert_eq!(heap.pop(), Some(3));
/// assert_eq!(heap.pop(), Some(2));
/// assert_eq!(heap.pop(), Some(1));
/// ```
pub struct Heap<T> {
  data: Vec<T>,
}

impl<T: Ord> Heap<T> {
  pub fn new() -> Self {
    Self { data: Vec::new() }
  }

  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }

  pub fn len(&self) -> usize {
    self.data.len()
  }

  pub fn peek(&self) -> Option<&T> {
    self.data.get(0)
  }

  pub fn push(&mut self, item: T) {
    let len = self.len();
    self.data.push(item);
    self.sift_up(len);
  }

  pub fn pop(&mut self) -> Option<T> {
    if self.is_empty() {
      None
    } else {
      let item = self.data.swap_remove(0);
      self.sift_down(0);
      Some(item)
    }
  }

  pub fn into_vec(self) -> Vec<T> {
    self.data
  }

  pub fn into_sorted_vec(mut self) -> Vec<T> {
    for i in (1..self.len()).rev() {
      self.data.swap(0, i);
      self.sift_down_range(0, i);
    }
    self.data
  }
}

impl<T: Ord> Default for Heap<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: Ord> From<Vec<T>> for Heap<T> {
  fn from(vec: Vec<T>) -> Self {
    let mut heap = Heap { data: vec };
    heap.rebuild();
    heap
  }
}

impl<T: Ord> Heap<T> {
  fn rebuild(&mut self) {
    for i in (0..(self.len() / 2)).rev() {
      self.sift_down(i);
    }
  }

  fn sift_up(&mut self, pos: usize) {
    if pos == 0 {
      return;
    }
    let parent = (pos - 1) / 2;
    if self.data[parent] < self.data[pos] {
      self.data.swap(pos, parent);
      self.sift_up(parent);
    }
  }

  fn sift_down(&mut self, pos: usize) {
    let len = self.len();
    self.sift_down_range(pos, len);
  }

  fn sift_down_range(&mut self, pos: usize, end: usize) {
    let left = pos * 2 + 1;
    let right = left + 1;
    let child = if right < end && self.data[left] < self.data[right] { right } else { left };
    if child < end && self.data[pos] < self.data[child] {
      self.data.swap(pos, child);
      self.sift_down_range(child, end);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Heap;

  #[test]
  fn push_pop() {
    let mut heap = Heap::new();
    for i in 0..10 {
      heap.push(i);
    }
    for i in (0..10).rev() {
      assert_eq!(heap.pop(), Some(i));
    }
    assert_eq!(heap.pop(), None);
  }

  #[test]
  fn from_vec() {
    let vec = vec![0, 2, 4, 6, 8, 9, 7, 5, 3, 1];
    let mut heap = Heap::from(vec);
    for i in (0..10).rev() {
      assert_eq!(heap.pop(), Some(i));
    }
    assert_eq!(heap.pop(), None);
  }

  #[test]
  fn into_sorted_vec() {
    let vec = vec![0, 2, 4, 6, 8, 9, 7, 5, 3, 1];
    let heap = Heap::from(vec);
    assert_eq!(heap.into_sorted_vec(), (0..10).collect::<Vec<_>>());
  }
}
