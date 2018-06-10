//! Least Recently Used

use cache::Cache;
use doublylist::DoublyList;
use doublylist::DoublyListNode;
use std::collections::HashMap;
use std::hash::Hash;
use std::mem;
use std::rc::Rc;

pub struct LRU<K, V> {
  capacity: usize,
  l: DoublyList<Rc<K>>,
  m: HashMap<Rc<K>, (DoublyListNode<Rc<K>>, V)>,
}

impl<K: Eq + Hash, V> Cache<K, V> for LRU<K, V> {
  fn get(&mut self, key: &K) -> Option<&V> {
    Some(self.get_mut(key)?)
  }

  fn set(&mut self, key: K, value: V) -> Option<V> {
    if let Some(old_value) = self.get_mut(&key) {
      return Some(mem::replace(old_value, value));
    }
    self.remove_old();
    self.insert_new(key, value);
    None
  }
}

impl<K: Eq + Hash, V> LRU<K, V> {
  pub fn new(capacity: usize) -> Self {
    LRU {
      capacity,
      l: DoublyList::new(),
      m: HashMap::with_capacity(capacity),
    }
  }

  pub fn is_empty(&self) -> bool {
    assert_eq!(self.m.is_empty(), self.l.is_empty());
    self.m.is_empty()
  }

  pub fn is_full(&self) -> bool {
    self.len() >= self.capacity
  }

  pub fn len(&self) -> usize {
    self.m.len()
  }

  pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
    let (node, value) = self.m.get_mut(key)?;
    refresh_node(node, &self.l);
    Some(value)
  }

  pub fn get_without_refresh(&self, key: &K) -> Option<&V> {
    Some(&self.m.get(key)?.1)
  }
}

impl<K: Eq + Hash, V> LRU<K, V> {
  fn insert_new(&mut self, key: K, value: V) {
    let key = Rc::new(key);
    let node = DoublyList::new_node(Rc::clone(&key));
    self.l.push_front_node(&node);
    self.m.insert(key, (node, value));
  }

  fn remove_old(&mut self) -> Option<()> {
    while self.is_full() {
      let node = self.l.pop_back_node()?;
      let key = &*DoublyList::node_data(&node);
      self.m.remove(key);
    }
    Some(())
  }
}

fn refresh_node<T>(node: &DoublyListNode<T>, list: &DoublyList<T>) {
  list.remove_node(node);
  list.push_front_node(node);
}

#[cfg(test)]
mod tests {
  use super::Cache;
  use super::LRU;

  #[test]
  fn set_get() {
    let mut cache = LRU::new(2);
    assert_eq!(cache.set("k".to_string(), "v".to_string()), None);
    assert_eq!(cache.get(&"k".to_string()), Some(&"v".to_string()));
    assert_eq!(cache.get_without_refresh(&"k".to_string()), Some(&"v".to_string()));
  }

  #[test]
  fn set_override() {
    let mut cache = LRU::new(2);
    assert_eq!(cache.set(1, 11), None);
    assert_eq!(cache.set(1, 12), Some(11));
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.get(&1), Some(&12));
    assert_eq!(cache.get_without_refresh(&1), Some(&12));
  }

  #[test]
  fn set_bounded() {
    let mut cache = LRU::new(2);
    cache.set(1, 11);
    cache.set(2, 22);
    cache.set(3, 33);
    assert_eq!(cache.len(), 2);
    assert_eq!(cache.get_without_refresh(&1), None);
    assert_eq!(cache.get_without_refresh(&2), Some(&22));
    assert_eq!(cache.get_without_refresh(&3), Some(&33));
  }

  #[test]
  fn set_refresh() {
    let mut cache = LRU::new(2);
    cache.set(1, 11);
    cache.set(2, 22);
    cache.set(1, 11);
    cache.set(3, 33);
    assert_eq!(cache.len(), 2);
    assert_eq!(cache.get_without_refresh(&1), Some(&11));
    assert_eq!(cache.get_without_refresh(&2), None);
    assert_eq!(cache.get_without_refresh(&3), Some(&33));
  }

  #[test]
  fn get_refresh() {
    let mut cache = LRU::new(2);
    cache.set(1, 11);
    cache.set(2, 22);
    cache.get(&1);
    cache.set(3, 33);
    assert_eq!(cache.len(), 2);
    assert_eq!(cache.get_without_refresh(&1), Some(&11));
    assert_eq!(cache.get_without_refresh(&2), None);
    assert_eq!(cache.get_without_refresh(&3), Some(&33));
  }

  #[test]
  fn len() {
    let mut cache = LRU::new(2);
    assert_eq!(cache.is_empty(), true);
    assert_eq!(cache.is_full(), false);
    assert_eq!(cache.len(), 0);
    cache.set(1, 11);
    assert_eq!(cache.is_empty(), false);
    assert_eq!(cache.is_full(), false);
    assert_eq!(cache.len(), 1);
    cache.set(2, 22);
    assert_eq!(cache.is_empty(), false);
    assert_eq!(cache.is_full(), true);
    assert_eq!(cache.len(), 2);
    cache.set(3, 33);
    assert_eq!(cache.is_empty(), false);
    assert_eq!(cache.is_full(), true);
    assert_eq!(cache.len(), 2);
  }
}
