/// Linked List
///
/// # Examples
///
/// ```
/// use structures::list;
///
/// let xs = list![1, 2, 3];
///
/// println!("{:?}", xs);
/// ```
#[derive(PartialEq)]
pub struct List<T> {
  head: Option<Box<Node<T>>>,
}

#[derive(PartialEq)]
struct Node<T> {
  next: Option<Box<Node<T>>>,
  data: T,
}

#[macro_export]
macro_rules! list {
  () => ($crate::list::List::nil());
  ($x:expr) => ($crate::list::List::cons($x, list![]));
  ($x:expr, $($xs:expr),*) => ($crate::list::List::cons($x, list![$($xs),*]));
}

impl<T> List<T> {
  pub fn nil() -> Self {
    List { head: None }
  }

  pub fn cons(data: T, mut next: Self) -> Self {
    let node = Node { data, next: next.head.take() };
    List { head: Some(Box::new(node)) }
  }

  pub fn decons(mut self) -> Option<(T, Self)> {
    self.head.take().map(|node| (node.data, List { head: node.next }))
  }

  pub fn is_empty(&self) -> bool {
    self.head.is_none()
  }

  pub fn len(&self) -> usize {
    let mut len = 0;
    let mut next = &self.head;
    while let Some(node) = next {
      next = &node.next;
      len += 1;
    }
    len
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut next = self.head.take();
    while let Some(mut node) = next {
      next = node.next.take();
    }
  }
}

impl<T: std::fmt::Debug> std::fmt::Debug for List<T> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.debug_list().entries(self.iter()).finish()
  }
}

mod iter {
  use super::{List, Node};

  pub struct Iter<'a, T>(&'a Option<Box<Node<T>>>);

  impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
      self.0.as_ref().map(|node| {
        self.0 = &node.next;
        &node.data
      })
    }
  }

  impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
      Iter(&self.head)
    }
  }

  pub struct IntoIter<T>(Option<Box<Node<T>>>);

  impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
      self.0.take().map(|node| {
        self.0 = node.next;
        node.data
      })
    }
  }

  impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(mut self) -> Self::IntoIter {
      IntoIter(self.head.take())
    }
  }
}

#[cfg(test)]
mod tests {
  use super::List;

  #[test]
  fn list_macro() {
    assert_eq!(list![], List::<()>::nil());
    assert_eq!(list![1], List::cons(1, List::nil()));
    assert_eq!(list![1, 2], List::cons(1, List::cons(2, List::nil())));
  }

  #[test]
  fn decons() {
    assert_eq!((list![] as List<()>).decons(), None);
    assert_eq!(list![1].decons(), Some((1, list![])));
    assert_eq!(list![1, 2].decons(), Some((1, list![2])));
  }

  #[test]
  fn is_empty() {
    assert!((list![] as List<()>).is_empty());
    assert!(!list![1].is_empty());
    assert!(!list![1, 2].is_empty());
  }

  #[test]
  fn len() {
    assert_eq!((list![] as List<()>).len(), 0);
    assert_eq!(list![1].len(), 1);
    assert_eq!(list![1, 2].len(), 2);
  }

  #[test]
  fn fmt() {
    assert_eq!(format!("{:?}", list![] as List<()>), "[]");
    assert_eq!(format!("{:?}", list![1]), "[1]");
    assert_eq!(format!("{:?}", list![1, 2]), "[1, 2]");
  }

  #[test]
  fn iter() {
    let h = |xs: List<_>| xs.iter().cloned().collect::<Vec<_>>();
    assert_eq!(h(list![]), []);
    assert_eq!(h(list![1]), [1]);
    assert_eq!(h(list![1, 2]), [1, 2]);
  }

  #[test]
  fn into_iter() {
    let h = |xs: List<_>| xs.into_iter().collect::<Vec<_>>();
    assert_eq!(h(list![]), []);
    assert_eq!(h(list![1]), [1]);
    assert_eq!(h(list![1, 2]), [1, 2]);
  }
}
