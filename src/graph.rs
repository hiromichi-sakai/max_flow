#![allow(dead_code)]

use std::collections::VecDeque;
use std::fmt::Debug;

pub type Flow = i64;

#[derive(Debug, PartialEq)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub flow: Flow,
    pub capacity: Flow,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InsideEdge {
    pub to: usize,
    pub flow: Flow,
    pub capacity: Flow,
    pub rev: usize,
}

impl InsideEdge {
    #[inline]
    pub fn residual_capacity(&self) -> Flow {
        assert!(self.flow <= self.capacity);
        self.capacity - self.flow
    }
}

// CSR format
#[derive(Default)]
pub struct Graph {
    num_nodes: usize,
    num_edges: usize,
    tmp_edge_list: Vec<Edge>,
    edge_index_to_inside_edge_index: Vec<usize>,
    tails: Vec<usize>,
    build_done: bool,

    pub start: Vec<usize>,
    pub inside_edge_list: Vec<InsideEdge>,

    pub excesses: Vec<Flow>,
    pub distances: Vec<usize>, // distance from u to sink in residual network

    que: VecDeque<usize>,
}

impl<'a> Graph {
    pub fn new() -> Self {
        Graph::default()
    }

    #[inline]
    pub fn num_nodes(&self) -> usize {
        self.num_nodes
    }

    #[inline]
    pub fn num_edges(&self) -> usize {
        self.num_edges
    }

    // return edge index
    pub fn add_directed_edge(&mut self, from: usize, to: usize, capacity: Flow) -> Option<usize> {
        if capacity <= 0 as Flow {
            return None;
        }
        self.tmp_edge_list.push(Edge {
            from,
            to,
            flow: 0 as Flow,
            capacity,
        });
        self.num_nodes = self.num_nodes.max(from.max(to) + 1);
        self.num_edges += 1;
        Some(self.num_edges - 1)
    }

    pub fn get_edge(&self, edge_index: usize) -> Edge {
        let e = &self.inside_edge_list[self.edge_index_to_inside_edge_index[edge_index]];
        Edge {
            from: self.tails[edge_index],
            to: e.to,
            flow: e.flow,
            capacity: e.capacity,
        }
    }

    pub fn clear(&mut self) {
        for edge_index in 0..self.num_edges {
            let inside_edge_index = self.edge_index_to_inside_edge_index[edge_index];
            self.inside_edge_list[inside_edge_index].flow = 0;
            let rev = self.inside_edge_list[inside_edge_index].rev;
            self.inside_edge_list[rev].flow = self.inside_edge_list[rev].capacity;
        }

        self.excesses.fill(0);
        self.distances.fill(self.num_nodes);
        self.que.clear();
    }

    pub fn neighbors(&'a self, u: usize) -> std::slice::Iter<'a, InsideEdge> {
        self.inside_edge_list[self.start[u]..self.start[u + 1]].iter()
    }

    pub fn build(&mut self) {
        if self.build_done {
            return;
        }
        self.build_done = true;
        // initialize
        self.edge_index_to_inside_edge_index.resize(self.num_edges, usize::MAX);
        self.tails.resize(self.num_edges, usize::MAX);
        self.start.resize(self.num_nodes + 1, 0);
        self.inside_edge_list.resize(2 * self.num_edges, InsideEdge::default());
        self.excesses.resize(self.num_nodes, 0);
        self.distances.resize(self.num_nodes, self.num_nodes);

        let mut degree = vec![0; self.num_nodes];
        for e in self.tmp_edge_list.iter() {
            degree[e.to] += 1;
            degree[e.from] += 1;
        }

        for i in 1..=self.num_nodes {
            self.start[i] += self.start[i - 1] + degree[i - 1];
        }

        let mut counter = vec![0; self.num_nodes];
        for (edge_index, e) in self.tmp_edge_list.iter().enumerate() {
            let (u, v) = (e.from, e.to);
            assert_ne!(u, v);
            let inside_edge_index_u = self.start[u] + counter[u];
            let inside_edge_index_v = self.start[v] + counter[v];
            counter[u] += 1;
            counter[v] += 1;

            // u -> v
            self.inside_edge_list[inside_edge_index_u] = InsideEdge {
                to: v,
                flow: 0,
                capacity: e.capacity,
                rev: inside_edge_index_v,
            };
            self.edge_index_to_inside_edge_index[edge_index] = inside_edge_index_u;
            self.tails[edge_index] = u;

            // v -> u
            self.inside_edge_list[inside_edge_index_v] = InsideEdge {
                to: u,
                flow: e.capacity,
                capacity: e.capacity,
                rev: inside_edge_index_u,
            };
        }
        self.tmp_edge_list.clear();
    }

    pub fn push_flow(&mut self, u: usize, edge_index: usize, flow: Flow) {
        if flow == 0 as Flow {
            return;
        }
        let to = self.inside_edge_list[edge_index].to;
        let rev = self.inside_edge_list[edge_index].rev;

        // update flow
        self.inside_edge_list[edge_index].flow += flow;
        self.inside_edge_list[rev].flow -= flow;

        // update excess
        self.excesses[u] -= flow;
        self.excesses[to] += flow;

        assert!(self.inside_edge_list[edge_index].flow <= self.inside_edge_list[edge_index].capacity);
        assert!(self.inside_edge_list[edge_index].flow >= 0 as Flow);
        assert!(self.inside_edge_list[rev].flow <= self.inside_edge_list[rev].capacity);
        assert!(self.inside_edge_list[rev].flow >= 0 as Flow);
    }

    // O(n + m)
    // calculate the distance from u to sink in the residual network
    // if such a path does not exist, distance[u] becomes self.num_nodes
    pub fn update_distance_to_sink(&mut self, source: usize, sink: usize) {
        self.que.clear();
        self.que.push_back(sink);
        self.distances.fill(self.num_nodes);
        self.distances[sink] = 0;

        while let Some(v) = self.que.pop_front() {
            for e in self.inside_edge_list[self.start[v]..self.start[v + 1]].iter() {
                // e.to -> v
                if e.flow > 0 as Flow && self.distances[e.to] > self.distances[v] + 1 {
                    self.distances[e.to] = self.distances[v] + 1;
                    if e.to != source {
                        self.que.push_back(e.to);
                    }
                }
            }
        }
    }

    #[inline]
    pub fn is_admissible_edge(&self, from: usize, i: usize) -> bool {
        self.inside_edge_list[i].residual_capacity() > 0 && self.distances[from] == self.distances[self.inside_edge_list[i].to] + 1
    }
}
