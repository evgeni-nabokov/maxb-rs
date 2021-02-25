use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Graph {
    vertices: HashMap<usize, HashSet<usize>>
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            vertices: HashMap::new()
        }
    }

    pub fn from_vec(edges: Vec<(usize, usize)>) -> Self {
        let mut res = Graph::new();
        for edge in edges {
            res.add_edge(edge)
        }
        res
    }

    pub fn add_vertex(&mut self, v: usize) {
        self.vertices.entry(v).or_insert(HashSet::new());
    }

    pub fn add_edge(&mut self, edge: (usize, usize)) {
        self.vertices.entry(edge.1).or_insert(HashSet::new());
        self.vertices.entry(edge.0).or_insert(HashSet::new()).insert(edge.1);
    }

    pub fn get_adj_vertices(&self, v: usize) -> Option<Vec<usize>> {
        self.vertices.get(&v)
            .map(|x| {
                let mut vec: Vec<usize> = x.iter().map(|y| *y).collect();
                vec.sort_unstable();
                vec
            })
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }
}