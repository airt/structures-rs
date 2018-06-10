//! Doubly Linked List
//!
//! # Examples
//!
//! ```
//! use structures::doublylist::DoublyList;
//!
//! let mut list = DoublyList::new();
//!
//! list.push_front(1);
//!
//! assert_eq!(list.pop_back(), Some(1));
//! ```

use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

pub struct DoublyList<T> {
  begin: DoublyListNode<T>,
  end: DoublyListNode<T>,
}

pub(crate) struct Node<T> {
  prev: Option<DoublyListWeakNode<T>>,
  next: Option<DoublyListNode<T>>,
  data: Option<T>,
}

pub(crate) type DoublyListNode<T> = Rc<RefCell<Node<T>>>;

type DoublyListWeakNode<T> = Weak<RefCell<Node<T>>>;

impl<T> Default for DoublyList<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> DoublyList<T> {
  pub fn new() -> Self {
    let begin = Node::new(None);
    let end = Node::new(None);
    Node::link(&begin, &end);
    DoublyList { begin, end }
  }

  pub fn len(&self) -> usize {
    let mut n = 0;
    let mut c = Rc::clone(&self.begin);
    while !Node::linking(&c, &self.end) {
      c = Node::next(&c);
      n += 1;
    }
    n
  }

  pub fn is_empty(&self) -> bool {
    Node::linking(&self.begin, &self.end)
  }

  pub fn push_front(&mut self, data: T) {
    self.push_front_node(&Self::new_node(data));
  }

  pub fn pop_back(&mut self) -> Option<T> {
    Some(Node::unwrap_data(self.pop_back_node()?).unwrap())
  }
}

impl<T> DoublyList<T> {
  pub(crate) fn push_front_node(&self, node: &DoublyListNode<T>) {
    Node::attach(&self.begin, node);
  }

  pub(crate) fn pop_back_node(&self) -> Option<DoublyListNode<T>> {
    if self.is_empty() {
      return None;
    }
    let node = Node::prev(&self.end);
    Node::detach(&node);
    Some(node)
  }

  pub(crate) fn new_node(data: T) -> DoublyListNode<T> {
    Node::new(Some(data))
  }

  pub(crate) fn remove_node(&self, node: &DoublyListNode<T>) {
    Node::detach(node)
  }

  pub(crate) fn node_data(node: &DoublyListNode<T>) -> Ref<T> {
    Node::borrow_data(node)
  }
}

impl<T> Node<T> {
  fn new(data: Option<T>) -> DoublyListNode<T> {
    Rc::new(RefCell::new(Node { prev: None, next: None, data }))
  }

  fn unwrap_data(node: DoublyListNode<T>) -> Option<T> {
    Some(Rc::try_unwrap(node).ok()?.into_inner().data.unwrap())
  }

  fn borrow_data(node: &DoublyListNode<T>) -> Ref<T> {
    Ref::map(node.borrow(), |x| x.data.as_ref().unwrap())
  }

  fn prev(node: &DoublyListNode<T>) -> DoublyListNode<T> {
    Rc::clone(&node.borrow().prev.as_ref().unwrap().upgrade().unwrap())
  }

  fn next(node: &DoublyListNode<T>) -> DoublyListNode<T> {
    Rc::clone(&node.borrow().next.as_ref().unwrap())
  }

  fn link(node: &DoublyListNode<T>, next: &DoublyListNode<T>) {
    node.borrow_mut().next = Some(Rc::clone(next));
    next.borrow_mut().prev = Some(Rc::downgrade(node));
  }

  fn attach(node: &DoublyListNode<T>, next: &DoublyListNode<T>) {
    Node::link(next, &Node::next(node));
    Node::link(node, next);
  }

  fn detach(node: &DoublyListNode<T>) {
    Node::link(&Node::prev(node), &Node::next(node));
    node.borrow_mut().prev = None;
    node.borrow_mut().next = None;
  }

  fn linking(node: &DoublyListNode<T>, next: &DoublyListNode<T>) -> bool {
    assert_eq!(Rc::ptr_eq(&Node::next(node), next), Rc::ptr_eq(&Node::prev(next), node));
    Rc::ptr_eq(&Node::next(node), next)
  }
}

#[cfg(test)]
mod tests {
  use super::DoublyList;
  use super::Node;

  #[test]
  fn node_link() {
    let node1 = DoublyList::new_node(1);
    let node2 = DoublyList::new_node(2);
    Node::link(&node1, &node2);
    assert!(node1.borrow().prev.is_none());
    assert!(node1.borrow().next.is_some());
    assert!(node2.borrow().prev.is_some());
    assert!(node2.borrow().next.is_none());
    assert!(Node::linking(&node1, &node2));
  }

  #[test]
  fn node_attach() {
    let node1 = DoublyList::new_node(1);
    let node2 = DoublyList::new_node(2);
    let node3 = DoublyList::new_node(3);
    Node::link(&node1, &node3);
    Node::attach(&node1, &node2);
    assert!(node1.borrow().prev.is_none());
    assert!(node1.borrow().next.is_some());
    assert!(node2.borrow().prev.is_some());
    assert!(node2.borrow().next.is_some());
    assert!(node3.borrow().prev.is_some());
    assert!(node3.borrow().next.is_none());
    assert!(Node::linking(&node1, &node2));
    assert!(Node::linking(&node2, &node3));
  }

  #[test]
  fn node_detach() {
    let node1 = DoublyList::new_node(1);
    let node2 = DoublyList::new_node(2);
    let node3 = DoublyList::new_node(3);
    Node::link(&node1, &node2);
    Node::link(&node2, &node3);
    Node::detach(&node2);
    assert!(node2.borrow().prev.is_none());
    assert!(node2.borrow().next.is_none());
    assert!(node1.borrow().prev.is_none());
    assert!(node1.borrow().next.is_some());
    assert!(node3.borrow().prev.is_some());
    assert!(node3.borrow().next.is_none());
    assert!(Node::linking(&node1, &node3));
  }

  #[test]
  fn push_pop() {
    let mut list = DoublyList::new();
    assert_eq!(list.pop_back(), None);
    list.push_front(1);
    list.push_front(2);
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), None);
  }

  #[test]
  fn len() {
    let mut list = DoublyList::new();
    assert_eq!(list.is_empty(), true);
    assert_eq!(list.len(), 0);
    list.push_front(1);
    assert_eq!(list.is_empty(), false);
    assert_eq!(list.len(), 1);
    list.push_front(2);
    assert_eq!(list.is_empty(), false);
    assert_eq!(list.len(), 2);
    list.pop_back();
    assert_eq!(list.is_empty(), false);
    assert_eq!(list.len(), 1);
    list.pop_back();
    assert_eq!(list.is_empty(), true);
    assert_eq!(list.len(), 0);
  }
}
