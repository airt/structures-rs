//! Linked List
//!
//! # Examples
//!
//! ```
//! # #[macro_use] extern crate structures;
//! # fn main() {
//! let xs = list![1, 2, 3];
//!
//! println!("{:?}", xs);
//! # }
//! ```

use std::fmt;

#[derive(PartialEq)]
pub enum List<T> {
  Nil,
  Cons(T, Box<List<T>>),
}

#[macro_export]
macro_rules! list {
  () => ($crate::list::List::Nil);
  ($x:expr) => ($crate::list::List::Cons($x, Box::new(list![])));
  ($x:expr, $($xs:expr),*) => ($crate::list::List::Cons($x, Box::new(list![$($xs),*])));
}

impl<T> List<T> {
  pub fn len(&self) -> usize {
    fn h<T>(zs: &List<T>, r: usize) -> usize {
      match zs {
        List::Nil => r,
        List::Cons(_, xs) => h(xs, r + 1),
      }
    }
    h(self, 0)
  }

  pub fn is_empty(&self) -> bool {
    match self {
      List::Nil => true,
      List::Cons(..) => false,
    }
  }

  pub fn iter(&self) -> Iter<T> {
    Iter(self)
  }
}

pub struct Iter<'a, T: 'a>(&'a List<T>);

impl<'a, T: 'a> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    match self.0 {
      List::Nil => None,
      List::Cons(x, xs) => {
        self.0 = xs;
        Some(x)
      }
    }
  }
}

impl<T: fmt::Debug> fmt::Debug for List<T> {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.debug_list().entries(self.iter()).finish()
  }
}

#[cfg(test)]
mod tests {
  use super::List::*;

  #[test]
  fn macro_list() {
    assert_eq!(list![], Nil::<()>);
    assert_eq!(list![1], Cons(1, Box::new(Nil)));
    assert_eq!(list![1, 2], Cons(1, Box::new(Cons(2, Box::new(Nil)))));
  }

  #[test]
  fn len() {
    assert_eq!(Nil::<()>.len(), 0);
    assert_eq!(list![1].len(), 1);
    assert_eq!(list![1, 2].len(), 2);
  }

  #[test]
  fn is_empty() {
    assert!(Nil::<()>.is_empty());
    assert!(!list![1].is_empty());
    assert!(!list![1, 2].is_empty());
  }

  #[test]
  fn iter() {
    let h = |xs: super::List<_>| xs.iter().cloned().collect::<Vec<_>>();
    assert_eq!(h(list![]), []);
    assert_eq!(h(list![1]), [1]);
    assert_eq!(h(list![1, 2]), [1, 2]);
  }

  #[test]
  fn fmt() {
    assert_eq!(format!("{:?}", Nil::<()>), "[]");
    assert_eq!(format!("{:?}", list![1]), "[1]");
    assert_eq!(format!("{:?}", list![1, 2]), "[1, 2]");
  }
}
