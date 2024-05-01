#![allow(dead_code)]
use crate::graph::{Flow, Graph};

pub struct BipartitePushRelabelHighestLabel<'a> {
    pub graph: &'a mut Graph,
    num_left_nodes: usize,
    _num_right_nodes: usize,
    alpha: usize,
    relabel_count: usize,

    buckets: Vec<Vec<usize>>, // buckets[i] = active nodes with distance i
    in_bucket: Vec<bool>,
    bucket_idx: usize,

    current_edge: Vec<usize>,
    distance_count: Vec<usize>,
}

impl<'a> BipartitePushRelabelHighestLabel<'a> {
    pub fn new(num_left_nodes: usize, num_right_nodes: usize, graph: &'a mut Graph) -> Self {
        assert!(num_left_nodes <= num_right_nodes);
        graph.build();
        let n = graph.num_nodes();

        BipartitePushRelabelHighestLabel {
            graph,
            num_left_nodes,
            _num_right_nodes: num_right_nodes,

            alpha: 0,
            relabel_count: 0,

            buckets: vec![Vec::new(); n],
            in_bucket: vec![false; n],
            bucket_idx: 0,

            current_edge: vec![0; n],
            distance_count: vec![0; n + 1],
        }
    }

    pub fn set_alpha(&mut self, alpha: usize) {
        self.alpha = alpha;
    }

    pub fn solve(&mut self, source: usize, sink: usize) -> Flow {
        if source == sink || self.graph.num_nodes() == 0 || self.graph.num_edges() == 0 {
            return 0 as Flow;
        }

        self.pre_process(source, sink);

        loop {
            if self.buckets[self.bucket_idx].is_empty() {
                if self.bucket_idx == 0 {
                    break;
                }
                self.bucket_idx -= 1;
                continue;
            }

            let u = self.buckets[self.bucket_idx].pop().unwrap();
            self.in_bucket[u] = false;
            self.bi_discharge(u, sink);
        }

        self.push_flow_excess_back_to_source(source, sink);

        self.graph.excesses[sink]
    }

    fn pre_process(&mut self, source: usize, sink: usize) {
        self.graph.update_distance_to_sink(source, sink);
        self.graph.distances[source] = 2 * self.num_left_nodes;

        for u in 0..self.graph.num_nodes() {
            self.distance_count[self.graph.distances[u]] += 1;
            self.current_edge[u] = self.graph.start[u];
        }

        for i in self.graph.start[source]..self.graph.start[source + 1] {
            let delta = self.graph.inside_edge_list[i].residual_capacity();
            self.graph.push_flow(source, i, delta);
        }

        for u in 0..self.graph.num_nodes() {
            if u != source && u != sink && self.graph.excesses[u] > 0 as Flow {
                self.enqueue(u);
            }
        }
        self.in_bucket[sink] = true;
    }

    fn enqueue(&mut self, u: usize) {
        if self.in_bucket[u] || self.graph.excesses[u] <= 0 as Flow || self.graph.distances[u] >= 2 * self.num_left_nodes {
            return;
        }

        self.in_bucket[u] = true;
        self.buckets[self.graph.distances[u]].push(u);
        self.bucket_idx = self.bucket_idx.max(self.graph.distances[u]);
    }

    fn bi_discharge(&mut self, u: usize, sink: usize) {
        // push u -> v -> w
        let mut u_has_admissible_edge = false;
        for i in self.current_edge[u]..self.graph.start[u + 1] {
            self.current_edge[u] = i;
            let v = self.graph.inside_edge_list[i].to;

            assert!(self.graph.excesses[u] > 0);
            if !self.graph.is_admissible_edge(u, i) {
                continue;
            }
            u_has_admissible_edge = true;

            let mut v_has_admissible_edge = false;
            for j in self.current_edge[v]..self.graph.start[v + 1] {
                self.current_edge[v] = j;
                let w = self.graph.inside_edge_list[j].to;

                if !self.graph.is_admissible_edge(v, j) {
                    continue;
                }
                v_has_admissible_edge = true;

                let delta = self.graph.excesses[u]
                    .min(self.graph.inside_edge_list[i].residual_capacity())
                    .min(self.graph.inside_edge_list[j].residual_capacity());
                assert!(delta > 0 as Flow);

                // push u -> v -> w
                self.graph.push_flow(u, i, delta);
                self.graph.push_flow(v, j, delta);

                if w != sink && self.graph.excesses[w] == delta {
                    self.enqueue(w);
                }

                if self.graph.excesses[u] == 0 as Flow {
                    return;
                }

                if self.graph.inside_edge_list[i].residual_capacity() == 0 as Flow {
                    break;
                }
            }

            // relabel
            if !v_has_admissible_edge {
                if self.distance_count[self.graph.distances[v]] == 1 {
                    self.gap_relabeling(self.graph.distances[v]);
                } else {
                    self.relabel(v);
                }
            }
            self.current_edge[v] = self.graph.start[v];
        }

        // relabel
        if !u_has_admissible_edge {
            if self.distance_count[self.graph.distances[u]] == 1 {
                self.gap_relabeling(self.graph.distances[u]);
            } else {
                self.relabel(u);
            }
        }
        self.current_edge[u] = self.graph.start[u];

        if self.graph.excesses[u] > 0 as Flow {
            self.enqueue(u);
        }
    }

    fn relabel(&mut self, u: usize) {
        self.relabel_count += 1;
        self.distance_count[self.graph.distances[u]] -= 1;

        self.graph.distances[u] = self
            .graph
            .neighbors(u)
            .filter(|edge| edge.residual_capacity() > 0 as Flow)
            .map(|edge| self.graph.distances[edge.to] + 1)
            .min()
            .unwrap_or(self.graph.num_nodes())
            .min(self.graph.num_nodes());

        self.distance_count[self.graph.distances[u]] += 1;
    }

    // gap relabeling heuristic
    fn gap_relabeling(&mut self, k: usize) {
        for u in 0..self.graph.num_nodes() {
            if self.graph.distances[u] >= k {
                self.distance_count[self.graph.distances[u]] -= 1;
                self.graph.distances[u] = self.graph.distances[u].max(self.graph.num_nodes());
                self.distance_count[self.graph.distances[u]] += 1;
            }
        }
    }

    fn push_flow_excess_back_to_source(&mut self, source: usize, sink: usize) {
        for u in 0..self.graph.num_nodes() {
            self.current_edge[u] = self.graph.start[u];
        }

        for u in 0..self.graph.num_nodes() {
            if u == source || u == sink {
                continue;
            }
            while self.graph.excesses[u] > 0 as Flow {
                // u から source への逆辺を使ったパスをみつける
                let mut visited = vec![false; self.graph.num_nodes()];
                self.dfs(u, source, sink, self.graph.excesses[u], &mut visited);
            }
            assert_eq!(self.graph.excesses[u], 0 as Flow);
        }
    }

    fn dfs(&mut self, u: usize, source: usize, sink: usize, flow: Flow, visited: &mut Vec<bool>) -> Flow {
        if u == source {
            return flow;
        }
        visited[u] = true;

        for i in self.current_edge[u]..self.graph.start[u + 1] {
            self.current_edge[u] = i;
            let to = self.graph.inside_edge_list[i].to;
            let residual_capacity = self.graph.inside_edge_list[i].residual_capacity();
            if visited[to] || residual_capacity == 0 as Flow {
                continue;
            }

            let delta = self.dfs(to, source, sink, flow.min(residual_capacity), visited);
            if delta > 0 as Flow {
                self.graph.push_flow(u, i, delta);
                return delta;
            }
        }
        0 as Flow
    }
}
