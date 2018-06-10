//! Cache
//!
//! # Examples
//!
//! ```
//! use structures::cache::Cache;
//! use structures::cache::lru::LRU;
//!
//! let mut cache = LRU::new(10);
//!
//! cache.set(1, 10);
//!
//! assert_eq!(cache.get(&1), Some(&10));
//! ```

pub mod lru;

pub trait Cache<K, V> {
  fn get(&mut self, key: &K) -> Option<&V>;
  fn set(&mut self, key: K, value: V) -> Option<V>;
}
