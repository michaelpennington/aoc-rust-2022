use std::{
    collections::{BinaryHeap, HashMap, VecDeque},
    hash::Hash,
};

/// A directed graph using an adjacency list approach.
/// `N` is the data stored in the node, `W` is the edge weight.
#[derive(Debug, Default)]
pub struct Graph<N, W = usize, K = usize>
where
    K: Eq + Hash + Clone,
{
    // Maps a unique ID to the node's data
    nodes: HashMap<K, N>,
    // Maps a Node ID to a map of its outbound edges (Target Node ID -> Weight)
    edges: HashMap<K, HashMap<K, W>>,
    // Generates the next unique Node ID
}

impl<N, W, K> Graph<N, W, K>
where
    K: Eq + Hash + Clone,
{
    /// Creates a new, empty graph.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    /// Adds a node with the given data and returns its unique ID.
    pub fn add_node(&mut self, key: K, data: N) -> Option<N> {
        self.edges.entry(key.clone()).or_default();
        self.nodes.insert(key, data)
    }

    /// Removes a node and all associated edges, returning the node's data.
    pub fn remove_node(&mut self, key: &K) -> Option<N> {
        let removed_node = self.nodes.remove(key);

        if removed_node.is_some() {
            // Remove outbound edges from this node
            self.edges.remove(key);

            // Remove inbound edges pointing to this node from other nodes
            for adj in self.edges.values_mut() {
                adj.remove(key);
            }
        }

        removed_node
    }

    /// Adds a directed edge between two nodes with a specific weight.
    pub fn add_edge(&mut self, from: K, to: K, weight: W) {
        // Ensure both nodes exist before adding an edge
        if self.nodes.contains_key(&from)
            && self.nodes.contains_key(&to)
            && let Some(adj) = self.edges.get_mut(&from)
        {
            adj.insert(to, weight);
        }
    }

    /// Removes an edge between two nodes, returning the weight if it existed.
    pub fn remove_edge(&mut self, from: &K, to: &K) -> Option<W> {
        self.edges.get_mut(from).and_then(|adj| adj.remove(to))
    }

    /// Returns an iterator over outgoing edges (target_id, weight).
    /// This is the exact method you will need for your A* loop.
    pub fn neighbors(&self, key: &K) -> Option<impl Iterator<Item = (&K, &W)>> {
        self.edges.get(key).map(|adj| adj.iter())
    }

    /// Retrieves a reference to a node's data.
    pub fn get_node(&self, key: &K) -> Option<&N> {
        self.nodes.get(key)
    }

    pub fn bfs(&self, key: &K) -> HashMap<K, usize> {
        let mut out = HashMap::with_capacity(self.nodes.len());
        let mut q = VecDeque::new();
        q.push_back((key.clone(), 0));
        out.insert(key.clone(), 0);
        while let Some((curr, dist)) = q.pop_front() {
            for (n, _) in self.neighbors(&curr).unwrap() {
                if !out.contains_key(n) {
                    out.insert(curr.clone(), dist + 1);
                    q.push_back((n.clone(), dist + 1));
                }
            }
        }
        out
    }
}
impl<N, K> Graph<N, usize, K>
where
    K: Eq + Hash + Clone,
{
    pub fn dijkstra(&self, key: &K) -> HashMap<K, usize> {
        let len = self.nodes.len();
        let mut out = HashMap::with_capacity(len);
        let mut pq = BinaryHeap::with_capacity(len);
        out.insert(key.clone(), 0);
        pq.push(Node { key, score: 0 });
        while let Some(Node { key, score }) = pq.pop() {
            let dist_u = out.get(key).copied().unwrap_or(usize::MAX);
            if score > dist_u {
                continue;
            }

            for (k, w) in self.neighbors(key).unwrap() {
                if dist_u + w < out.get(k).copied().unwrap_or(usize::MAX) {
                    out.insert(k.clone(), dist_u + w);
                    pq.push(Node {
                        key: k,
                        score: dist_u + w,
                    });
                }
            }
        }

        out
    }
}

#[derive(Debug, Clone, Copy)]
struct Node<K> {
    key: K,
    score: usize,
}

impl<K> PartialEq for Node<K> {
    fn eq(&self, other: &Self) -> bool {
        self.score.eq(&other.score)
    }
}

impl<K> Eq for Node<K> {}

impl<K> PartialOrd for Node<K> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<K> Ord for Node<K> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}
