use std::cell::{Ref, RefCell};
use std::iter::FromIterator;
use std::rc::Rc;

/// Doubly Linked List
///
/// # Examples
///
/// ```
/// use structures::doublylist::DoublyList;
///
/// let mut list = DoublyList::new();
///
/// list.push_front(1);
///
/// assert_eq!(list.pop_back(), Some(1));
/// ```
pub struct DoublyList<T> {
  head: Option<NodeRef<T>>,
  tail: Option<NodeRef<T>>,
  len: usize,
}

pub(crate) struct Node<T> {
  prev: Option<NodeRef<T>>,
  next: Option<NodeRef<T>>,
  data: T,
}

pub(crate) type NodeRef<T> = Rc<RefCell<Node<T>>>;

impl<T> DoublyList<T> {
  pub fn new() -> Self {
    Self { head: None, tail: None, len: 0 }
  }

  pub fn is_empty(&self) -> bool {
    debug_assert_eq!(self.head.is_none(), self.tail.is_none());
    self.head.is_none()
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn front(&self) -> Option<Ref<T>> {
    self.head.as_ref().map(Self::borrow_node_data)
  }

  pub fn back(&self) -> Option<Ref<T>> {
    self.tail.as_ref().map(Self::borrow_node_data)
  }

  pub fn push_front(&mut self, data: T) {
    self.push_front_node(Self::new_node(data));
  }

  pub fn push_back(&mut self, data: T) {
    self.push_back_node(Self::new_node(data));
  }

  pub fn pop_front(&mut self) -> Option<T> {
    self.pop_front_node().map(Self::unwrap_node_data)
  }

  pub fn pop_back(&mut self) -> Option<T> {
    self.pop_back_node().map(Self::unwrap_node_data)
  }

  pub(crate) fn push_front_node(&mut self, node: NodeRef<T>) {
    match self.head.take() {
      Some(head) => {
        head.borrow_mut().prev = Some(node.clone());
        node.borrow_mut().next = Some(head);
        self.head = Some(node);
      }
      None => {
        self.head = Some(node.clone());
        self.tail = Some(node);
      }
    }
    self.len += 1;
  }

  pub(crate) fn push_back_node(&mut self, node: NodeRef<T>) {
    match self.tail.take() {
      Some(tail) => {
        tail.borrow_mut().next = Some(node.clone());
        node.borrow_mut().prev = Some(tail);
        self.tail = Some(node);
      }
      None => {
        self.head = Some(node.clone());
        self.tail = Some(node);
      }
    }
    self.len += 1;
  }

  pub(crate) fn pop_front_node(&mut self) -> Option<NodeRef<T>> {
    self.head.take().map(|head| {
      match head.borrow_mut().next.take() {
        Some(node) => {
          node.borrow_mut().prev = None;
          self.head = Some(node);
        }
        None => self.tail = None,
      }
      self.len -= 1;
      head
    })
  }

  pub(crate) fn pop_back_node(&mut self) -> Option<NodeRef<T>> {
    self.tail.take().map(|tail| {
      match tail.borrow_mut().prev.take() {
        Some(node) => {
          node.borrow_mut().next = None;
          self.tail = Some(node);
        }
        None => self.head = None,
      }
      self.len -= 1;
      tail
    })
  }

  pub(crate) fn new_node(data: T) -> NodeRef<T> {
    Rc::new(RefCell::new(Node { prev: None, next: None, data }))
  }

  pub(crate) fn unwrap_node_data(node: NodeRef<T>) -> T {
    Rc::try_unwrap(node).ok().unwrap().into_inner().data
  }

  pub(crate) fn borrow_node_data(node: &NodeRef<T>) -> Ref<T> {
    Ref::map(node.borrow(), |node| &node.data)
  }

  pub(crate) fn unlink_node(&mut self, node: &NodeRef<T>) {
    let mut node = node.borrow_mut();
    let prev = node.prev.take();
    let next = node.next.take();
    match prev.clone() {
      Some(prev) => prev.borrow_mut().next = next.clone(),
      None => self.head = next.clone(),
    }
    match next {
      Some(next) => next.borrow_mut().prev = prev,
      None => self.tail = prev,
    }
    self.len -= 1;
  }
}

impl<T> Drop for DoublyList<T> {
  fn drop(&mut self) {
    while self.pop_front_node().is_some() {}
  }
}

impl<T> Default for DoublyList<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> Extend<T> for DoublyList<T> {
  fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
    for x in iter {
      self.push_back(x)
    }
  }
}

impl<T> FromIterator<T> for DoublyList<T> {
  fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    let mut list = Self::new();
    list.extend(iter);
    list
  }
}

impl<T> IntoIterator for DoublyList<T> {
  type Item = T;
  type IntoIter = IntoIter<T>;
  fn into_iter(self) -> Self::IntoIter {
    IntoIter(self)
  }
}

pub struct IntoIter<T>(DoublyList<T>);

impl<T> Iterator for IntoIter<T> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop_front()
  }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    self.0.pop_back()
  }
}

#[cfg(test)]
mod tests {
  use super::DoublyList;

  #[test]
  fn new() {
    let list = DoublyList::<()>::new();
    assert_eq!(list.len(), 0);
    assert_eq!(list.is_empty(), true);
    assert_eq!(list.front().map(|x| *x), None);
    assert_eq!(list.back().map(|x| *x), None);
  }

  #[test]
  fn push_front() {
    let mut list = DoublyList::new();
    list.push_front(3);
    assert_eq!(list.len(), 1);
    assert_eq!(list.is_empty(), false);
    assert_eq!(list.front().map(|x| *x), Some(3));
    assert_eq!(list.back().map(|x| *x), Some(3));
    list.push_front(2);
    assert_eq!(list.len(), 2);
    assert_eq!(list.is_empty(), false);
    assert_eq!(list.front().map(|x| *x), Some(2));
    assert_eq!(list.back().map(|x| *x), Some(3));
    list.push_front(1);
    assert_eq!(list.len(), 3);
    assert_eq!(list.is_empty(), false);
    assert_eq!(list.front().map(|x| *x), Some(1));
    assert_eq!(list.back().map(|x| *x), Some(3));
  }

  #[test]
  fn push_back() {
    let mut list = DoublyList::new();
    list.push_back(1);
    assert_eq!(list.len(), 1);
    assert_eq!(list.is_empty(), false);
    assert_eq!(list.front().map(|x| *x), Some(1));
    assert_eq!(list.back().map(|x| *x), Some(1));
    list.push_back(2);
    assert_eq!(list.len(), 2);
    assert_eq!(list.is_empty(), false);
    assert_eq!(list.front().map(|x| *x), Some(1));
    assert_eq!(list.back().map(|x| *x), Some(2));
    list.push_back(3);
    assert_eq!(list.len(), 3);
    assert_eq!(list.is_empty(), false);
    assert_eq!(list.front().map(|x| *x), Some(1));
    assert_eq!(list.back().map(|x| *x), Some(3));
  }

  #[test]
  fn pop_front() {
    let mut list = DoublyList::new();
    assert_eq!(list.pop_front(), None);
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    assert_eq!(list.len(), 3);
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.len(), 2);
    assert_eq!(list.is_empty(), false);
    assert_eq!(list.front().map(|x| *x), Some(2));
    assert_eq!(list.back().map(|x| *x), Some(3));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.len(), 1);
    assert_eq!(list.is_empty(), false);
    assert_eq!(list.front().map(|x| *x), Some(3));
    assert_eq!(list.back().map(|x| *x), Some(3));
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.len(), 0);
    assert_eq!(list.is_empty(), true);
    assert_eq!(list.front().map(|x| *x), None);
    assert_eq!(list.back().map(|x| *x), None);
  }

  #[test]
  fn pop_back() {
    let mut list = DoublyList::new();
    assert_eq!(list.pop_back(), None);
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.len(), 3);
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.len(), 2);
    assert_eq!(list.is_empty(), false);
    assert_eq!(list.front().map(|x| *x), Some(1));
    assert_eq!(list.back().map(|x| *x), Some(2));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.len(), 1);
    assert_eq!(list.is_empty(), false);
    assert_eq!(list.front().map(|x| *x), Some(1));
    assert_eq!(list.back().map(|x| *x), Some(1));
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.len(), 0);
    assert_eq!(list.is_empty(), true);
    assert_eq!(list.front().map(|x| *x), None);
    assert_eq!(list.back().map(|x| *x), None);
  }

  #[test]
  fn into_iter() {
    let list = DoublyList::<()>::new();
    assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![]);
    let mut list = DoublyList::new();
    list.push_back(1);
    assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1]);
    let mut list = DoublyList::new();
    list.push_back(1);
    list.push_back(2);
    assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1, 2]);
    let mut list = DoublyList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1, 2, 3]);
  }

  #[test]
  fn from_iter() {
    let list = vec![].into_iter().collect::<DoublyList<()>>();
    assert_eq!(list.len(), 0);
    assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![]);
    let list = vec![1].into_iter().collect::<DoublyList<_>>();
    assert_eq!(list.len(), 1);
    assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1]);
    let list = vec![1, 2].into_iter().collect::<DoublyList<_>>();
    assert_eq!(list.len(), 2);
    assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1, 2]);
    let list = vec![1, 2, 3].into_iter().collect::<DoublyList<_>>();
    assert_eq!(list.len(), 3);
    assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1, 2, 3]);
  }
}
