use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::rc::Rc;

/// Directed Graph
///
/// # Examples
///
/// ```
/// use structures::graph::Graph;
///
/// let graph: Graph = vec![
///   (1, 2, 7), (1, 3, 9), (1, 6, 14),
///   (2, 3, 10), (2, 4, 15), (3, 4, 11),
///   (3, 6, 2), (4, 5, 6), (6, 5, 9),
/// ].into_iter().collect();
///
/// assert_eq!(graph.shortest_path(1, Some(5))[&5], (20, vec![1, 3, 6, 5]));
///
/// assert_eq!(graph.shortest_paths()[&1][&5], (20, vec![1, 3, 6, 5]));
/// ```
///
/// ![dijkstra-animation](https://upload.wikimedia.org/wikipedia/commons/5/57/Dijkstra_Animation.gif)
#[derive(Default)]
pub struct Graph {
  data: HashMap<Vertex, HashMap<Vertex, Weight>>,
}

pub type Weight = usize;
pub type Vertex = usize;
pub type Vertices = HashSet<Vertex>;
pub type Edge = (Vertex, Vertex, Weight);
pub type Edges = Vec<Edge>;
pub type Path = Vec<Vertex>;

impl Graph {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn vertices(&self) -> Vertices {
    self.data.keys().chain(self.data.values().flat_map(HashMap::keys)).cloned().collect()
  }

  pub fn vertices_outgoing_from(&self, source: Vertex) -> Vertices {
    self.data.get(&source).map(HashMap::keys).map(|t| t.cloned().collect()).unwrap_or_default()
  }

  pub fn edges(&self) -> Edges {
    self.data.iter().flat_map(|(x, t)| t.iter().map(move |(z, w)| (*x, *z, *w))).collect()
  }

  pub fn get_weight(&self, source: Vertex, target: Vertex) -> Option<Weight> {
    self.data.get(&source)?.get(&target).cloned()
  }

  pub fn add_edge(&mut self, source: Vertex, target: Vertex, weight: Weight) {
    self.data.entry(source).or_insert_with(HashMap::new).insert(target, weight);
  }
}

impl Extend<(Vertex, Vertex, Weight)> for Graph {
  fn extend<I: IntoIterator<Item = (Vertex, Vertex, Weight)>>(&mut self, iter: I) {
    for x in iter {
      self.add_edge(x.0, x.1, x.2)
    }
  }
}

impl FromIterator<(Vertex, Vertex, Weight)> for Graph {
  fn from_iter<I: IntoIterator<Item = (Vertex, Vertex, Weight)>>(iter: I) -> Self {
    let mut graph = Self::new();
    graph.extend(iter);
    graph
  }
}

impl Graph {
  /// Dijkstra's Algorithm
  pub fn shortest_path(&self, source: Vertex, target: Option<Vertex>) -> HashMap<Vertex, (Weight, Path)> {
    let mut weights = HashMap::<Vertex, Weight>::new();
    let mut previous = HashMap::<Vertex, Vertex>::new();
    let mut vertices_done = HashSet::<Vertex>::new();
    let mut vertices_todo = BinaryHeap::<Reverse<(Weight, Vertex)>>::new();

    weights.insert(source, 0);
    vertices_todo.push(Reverse((0, source)));

    while let Some(Reverse((_, u))) = vertices_todo.pop() {
      if Some(u) == target {
        break;
      }

      vertices_done.insert(u);

      for &v in self.vertices_outgoing_from(u).difference(&vertices_done) {
        let new_weight = weights[&u] + self.get_weight(u, v).unwrap();
        if !weights.contains_key(&v) || weights[&v] > new_weight {
          weights.insert(v, new_weight);
          previous.insert(v, u);
          vertices_todo.push(Reverse((new_weight, v)));
        } else {
          vertices_todo.push(Reverse((weights[&v], v)));
        }
      }
    }

    let resolve = |v| -> Path {
      let mut path = VecDeque::new();
      path.push_front(v);
      while path[0] != source {
        let pv = previous[&path[0]];
        path.push_front(pv);
      }
      path.into()
    };

    let vertices = target.map(|t| vec![t].into_iter().collect()).unwrap_or_else(|| self.vertices());

    vertices.intersection(&weights.keys().cloned().collect()).map(|&v| (v, (weights[&v], resolve(v)))).collect()
  }

  /// Floyd-Warshall Algorithm
  pub fn shortest_paths(&self) -> HashMap<Vertex, HashMap<Vertex, (Weight, Path)>> {
    type M<K, V> = HashMap<K, HashMap<K, V>>;
    let put = |m: &mut M<_, _>, x, y, v| m.entry(x).or_insert_with(HashMap::new).insert(y, v);

    let mut e = M::<_, _>::new();
    for x in self.vertices() {
      put(&mut e, x, x, (0, vec![x]));
    }
    for (x, z, w) in self.edges() {
      put(&mut e, x, z, (w, vec![x, z]));
    }

    let e = Rc::new(RefCell::new(e));
    {
      let get = |x, z| e.borrow().get(&x).and_then(|t| t.get(&z)).cloned();
      let set = |x, z, t| put(&mut e.borrow_mut(), x, z, t);
      let vs = self.vertices();
      for &k in &vs {
        for &i in &vs {
          for &j in &vs {
            if let Some((w1, p1)) = get(i, k) {
              if let Some((w2, p2)) = get(k, j) {
                match get(i, j) {
                  Some((w0, _)) if w0 <= w1 + w2 => {}
                  _ => {
                    let w = w1 + w2;
                    let p = p1.into_iter().chain(p2.into_iter().skip(1)).collect();
                    set(i, j, (w, p));
                  }
                }
              }
            }
          }
        }
      }
    }

    Rc::try_unwrap(e).unwrap().into_inner()
  }

  /// Topological Sorting
  pub fn topo_sort(&self) -> Option<Path> {
    let mut vertices_incoming_to = HashMap::new();
    for (x, z, _) in self.edges() {
      vertices_incoming_to.entry(z).or_insert_with(HashSet::new).insert(x);
    }
    let no_indegree_in = |v: &_, vs: &_| match vertices_incoming_to.get(v) {
      Some(ins) => ins.is_disjoint(vs),
      None => true,
    };

    let mut sorted = vec![];
    let mut vertices_todo = self.vertices();

    while !vertices_todo.is_empty() {
      let ts = vertices_todo.iter().filter(|v| no_indegree_in(v, &vertices_todo)).cloned().collect::<HashSet<_>>();
      if ts.is_empty() {
        return None;
      }
      for t in &ts {
        vertices_todo.remove(t);
      }
      sorted.extend(ts);
    }

    Some(sorted)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vertices() {
    let graph = new_graph();
    assert_eq!(graph.vertices(), vec![1, 2, 3, 4, 5, 6].into_iter().collect());
  }

  #[test]
  fn vertices_outgoing_from() {
    let graph = new_graph();
    assert_eq!(graph.vertices_outgoing_from(1), vec![2, 3, 6].into_iter().collect());
  }

  #[test]
  fn edges() {
    assert_eq!(new_graph().edges().len(), 9);
    assert_eq!(new_undirected_graph().edges().len(), 9 * 2);
  }

  #[test]
  fn shortest_path() {
    let graph = new_graph();
    let paths = vec![
      (1, (0, vec![1])),
      (2, (7, vec![1, 2])),
      (3, (9, vec![1, 3])),
      (4, (20, vec![1, 3, 4])),
      (5, (20, vec![1, 3, 6, 5])),
      (6, (11, vec![1, 3, 6])),
    ];
    assert_eq!(graph.shortest_path(1, None), paths.into_iter().collect());
    assert_eq!(graph.shortest_path(1, None).len(), graph.vertices().len());
    assert_eq!(graph.shortest_path(1, Some(2)).len(), 1);
  }

  #[test]
  fn shortest_paths() {
    let graph = new_graph();
    let ow = |m: HashMap<_, _>| m.into_iter().map(|(z, (w, _))| (z, w)).collect::<HashMap<_, _>>();
    for (x, t) in graph.shortest_paths() {
      assert_eq!(ow(graph.shortest_path(x, None)), ow(t));
    }
  }

  #[test]
  fn topo_sort() {
    let sorted = new_undirected_graph().topo_sort();
    assert!(sorted.is_none());
    let sorted = new_graph().topo_sort();
    assert!(sorted.is_some());
    let sorted = sorted.unwrap();
    assert!(sorted == [1, 2, 3, 4, 6, 5] || sorted == [1, 2, 3, 6, 4, 5]);
  }

  fn new_graph() -> Graph {
    new_graph_edges().into_iter().collect()
  }

  fn new_undirected_graph() -> Graph {
    new_graph_edges().into_iter().flat_map(|t| vec![t, (t.1, t.0, t.2)]).collect()
  }

  fn new_graph_edges() -> Edges {
    vec![(1, 2, 7), (1, 3, 9), (1, 6, 14), (2, 3, 10), (2, 4, 15), (3, 4, 11), (3, 6, 2), (4, 5, 6), (6, 5, 9)]
  }
}
