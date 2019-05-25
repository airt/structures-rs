/// Binary Tree
///
/// # Examples
///
/// ```
/// use structures::tree::{TraverseOrder, Tree};
///
/// let tree = Tree::branch(1, Tree::leaf(2), Tree::leaf(3));
///
/// println!("{:#?}", tree);
///
/// println!("{:?}", tree.iter(TraverseOrder::InOrder).collect::<Vec<_>>());
/// ```
#[derive(Debug, PartialEq)]
pub enum Tree<T> {
  Empty,
  Branch(T, Box<Self>, Box<Self>),
}

impl<T> Tree<T> {
  pub fn empty() -> Self {
    Tree::Empty
  }

  pub fn leaf(v: T) -> Self {
    Tree::branch(v, Tree::Empty, Tree::Empty)
  }

  pub fn branch(v: T, l: Self, r: Self) -> Self {
    Tree::Branch(v, Box::new(l), Box::new(r))
  }

  pub fn left(&self) -> Option<&Self> {
    match self {
      Tree::Empty => None,
      Tree::Branch(_, l, _) => Some(&l),
    }
  }

  pub fn right(&self) -> Option<&Self> {
    match self {
      Tree::Empty => None,
      Tree::Branch(_, _, r) => Some(&r),
    }
  }

  pub fn value(&self) -> Option<&T> {
    match self {
      Tree::Empty => None,
      Tree::Branch(v, _, _) => Some(&v),
    }
  }
}

#[derive(Clone, Copy)]
pub enum TraverseOrder {
  InOrder,
  PreOrder,
  PostOrder,
}

mod iter {
  use super::{TraverseOrder, Tree};

  pub struct Iter<'a, T> {
    stack: Vec<IterState<'a, T>>,
    order: TraverseOrder,
  }

  enum IterState<'a, T> {
    T(&'a Tree<T>),
    V(&'a T),
  }

  impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
      use {IterState::*, TraverseOrder::*, Tree::*};
      let Iter { order, stack } = self;
      while let Some(state) = stack.pop() {
        match state {
          V(v) => {
            return Some(v);
          }
          T(Empty) => {}
          T(Branch(v, l, r)) => {
            if let PostOrder = order {
              stack.push(V(v))
            }
            stack.push(T(r));
            if let InOrder = order {
              stack.push(V(v))
            }
            stack.push(T(l));
            if let PreOrder = order {
              stack.push(V(v))
            }
          }
        }
      }
      None
    }
  }

  impl<T> Tree<T> {
    pub fn iter(&self, order: TraverseOrder) -> Iter<T> {
      Iter {
        stack: vec![IterState::T(self)],
        order,
      }
    }
  }

  impl<T> Tree<T> {
    pub fn traverse(&self, order: TraverseOrder, f: &mut FnMut(&T)) {
      use {TraverseOrder::*, Tree::*};
      match self {
        Empty => {}
        Branch(ref v, l, r) => {
          if let PreOrder = order {
            f(v)
          }
          l.traverse(order, f);
          if let InOrder = order {
            f(v)
          }
          r.traverse(order, f);
          if let PostOrder = order {
            f(v)
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Tree;

  #[test]
  fn left() {
    assert_eq!(Tree::<()>::empty().left(), None);
    assert_eq!(Tree::branch(1, new_tree(), Tree::empty()).left(), Some(&new_tree()));
  }

  #[test]
  fn right() {
    assert_eq!(Tree::<()>::empty().right(), None);
    assert_eq!(Tree::branch(1, Tree::empty(), new_tree()).right(), Some(&new_tree()));
  }

  #[test]
  fn value() {
    assert_eq!(Tree::<()>::empty().value(), None);
    assert_eq!(Tree::leaf(1).value(), Some(&1));
  }

  #[test]
  fn iter() {
    use super::TraverseOrder::*;
    let h = |o| new_tree().iter(o).cloned().collect::<Vec<_>>();
    assert_eq!(h(InOrder), [4, 2, 5, 1, 6, 3, 7]);
    assert_eq!(h(PreOrder), [1, 2, 4, 5, 3, 6, 7]);
    assert_eq!(h(PostOrder), [4, 5, 2, 6, 7, 3, 1]);
  }

  #[test]
  fn traverse() {
    use super::TraverseOrder::*;
    let h = |o| {
      let mut rs = vec![];
      new_tree().traverse(o, &mut |&x| rs.push(x));
      rs
    };
    assert_eq!(h(InOrder), [4, 2, 5, 1, 6, 3, 7]);
    assert_eq!(h(PreOrder), [1, 2, 4, 5, 3, 6, 7]);
    assert_eq!(h(PostOrder), [4, 5, 2, 6, 7, 3, 1]);
  }

  fn new_tree() -> Tree<i32> {
    Tree::branch(1, Tree::branch(2, Tree::leaf(4), Tree::leaf(5)), Tree::branch(3, Tree::leaf(6), Tree::leaf(7)))
  }
}
