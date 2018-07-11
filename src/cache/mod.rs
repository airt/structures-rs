pub mod lru;

pub trait Cache<K, V> {
  fn get(&mut self, key: &K) -> Option<&V>;
  fn set(&mut self, key: K, value: V) -> Option<V>;
}
