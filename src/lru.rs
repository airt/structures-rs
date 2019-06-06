use std::collections::HashMap;
use std::hash::Hash;
use std::mem;
use std::rc::Rc;

use crate::doublylist::{DoublyList, NodeRef};

/// Least Recently Used Cache
///
/// # Examples
///
/// ```
/// use structures::lru::LruCache;
///
/// let mut cache = LruCache::with_capacity(10);
///
/// cache.insert(1, 10);
///
/// assert_eq!(cache.get(&1), Some(&10));
/// ```
pub struct LruCache<K, V> {
  capacity: usize,
  list: DoublyList<Rc<K>>,
  map: HashMap<Rc<K>, (NodeRef<Rc<K>>, V)>,
}

impl<K: Eq + Hash, V> LruCache<K, V> {
  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      capacity,
      list: DoublyList::new(),
      map: HashMap::with_capacity(capacity),
    }
  }

  pub fn is_empty(&self) -> bool {
    debug_assert_eq!(self.list.is_empty(), self.map.is_empty());
    self.list.is_empty()
  }

  pub fn len(&self) -> usize {
    debug_assert_eq!(self.list.len(), self.map.len());
    self.list.len()
  }

  pub fn contains(&self, key: &K) -> bool {
    self.map.contains_key(key)
  }

  pub fn peek(&self, key: &K) -> Option<&V> {
    self.map.get(key).map(|(_, value)| value)
  }

  pub fn get(&mut self, key: &K) -> Option<&V> {
    self.get_mut(key).map(|value| &*value)
  }

  pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
    let (node, value) = self.map.get_mut(key)?;
    self.list.unlink_node(node);
    self.list.push_front_node(node.clone());
    value.into()
  }

  pub fn insert(&mut self, key: K, value: V) -> Option<V> {
    if let Some(old_value) = self.get_mut(&key) {
      return Some(mem::replace(old_value, value));
    }
    if self.len() >= self.capacity {
      self.remove_lru();
    }
    let key = Rc::new(key);
    let node = DoublyList::new_node(key.clone());
    self.list.push_front_node(node.clone());
    self.map.insert(key, (node, value));
    None
  }

  pub fn remove(&mut self, key: &K) -> Option<V> {
    let (node, value) = self.map.remove(key)?;
    self.list.unlink_node(&node);
    value.into()
  }

  fn remove_lru(&mut self) {
    if let Some(node) = self.list.pop_back_node() {
      let key = &*DoublyList::borrow_node_data(&node);
      self.map.remove(key);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::LruCache;

  #[test]
  fn new() {
    let cache = LruCache::<(), ()>::with_capacity(2);
    assert_eq!(cache.is_empty(), true);
    assert_eq!(cache.len(), 0);
  }

  #[test]
  fn insert() {
    let mut cache = LruCache::with_capacity(2);
    assert_eq!(cache.contains(&"k".to_string()), false);
    assert_eq!(cache.peek(&"k".to_string()), None);
    assert_eq!(cache.insert("k".to_string(), "v".to_string()), None);
    assert_eq!(cache.contains(&"k".to_string()), true);
    assert_eq!(cache.peek(&"k".to_string()), Some(&"v".to_string()));
  }

  #[test]
  fn insert_override() {
    let mut cache = LruCache::with_capacity(2);
    assert_eq!(cache.insert(1, 11), None);
    assert_eq!(cache.insert(1, 12), Some(11));
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.peek(&1), Some(&12));
  }

  #[test]
  fn insert_bounded() {
    let mut cache = LruCache::with_capacity(2);
    cache.insert(1, 11);
    cache.insert(2, 22);
    cache.insert(3, 33);
    assert_eq!(cache.len(), 2);
    assert_eq!(cache.peek(&1), None);
    assert_eq!(cache.peek(&2), Some(&22));
    assert_eq!(cache.peek(&3), Some(&33));
  }

  #[test]
  fn insert_refresh() {
    let mut cache = LruCache::with_capacity(2);
    cache.insert(1, 11);
    cache.insert(2, 22);
    cache.insert(1, 11);
    cache.insert(3, 33);
    assert_eq!(cache.len(), 2);
    assert_eq!(cache.peek(&1), Some(&11));
    assert_eq!(cache.peek(&2), None);
    assert_eq!(cache.peek(&3), Some(&33));
  }

  #[test]
  fn get_refresh() {
    let mut cache = LruCache::with_capacity(2);
    cache.insert(1, 11);
    cache.insert(2, 22);
    assert_eq!(cache.get(&1), Some(&11));
    cache.insert(3, 33);
    assert_eq!(cache.len(), 2);
    assert_eq!(cache.peek(&1), Some(&11));
    assert_eq!(cache.peek(&2), None);
    assert_eq!(cache.peek(&3), Some(&33));
  }

  #[test]
  fn remove() {
    let mut cache = LruCache::with_capacity(2);
    assert_eq!(cache.remove(&1), None);
    assert_eq!(cache.len(), 0);
    assert_eq!(cache.insert(1, 11), None);
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.remove(&1), Some(11));
    assert_eq!(cache.len(), 0);
    assert_eq!(cache.remove(&1), None);
    assert_eq!(cache.len(), 0);
  }
}
