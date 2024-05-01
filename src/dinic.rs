#![allow(dead_code)]
use crate::graph::{Flow, Graph};

pub struct Dinic<'a> {
    pub graph: &'a mut Graph,
    current_edge: Vec<usize>,
}

impl<'a> Dinic<'a> {
    pub fn new(graph: &'a mut Graph) -> Self {
        let n = graph.num_nodes();
        Dinic {
            graph,
            current_edge: vec![0; n],
        }
    }

    pub fn solve(&mut self, source: usize, sink: usize) -> Flow {
        self.graph.build();
        if source == sink || self.graph.num_nodes() == 0 || self.graph.num_edges() == 0 {
            return 0 as Flow;
        }

        loop {
            self.graph.update_distance_to_sink(source, sink);

            // no s-t path
            if self.graph.distances[source] >= self.graph.num_nodes() {
                break;
            }

            self.current_edge.iter_mut().enumerate().for_each(|(u, e)| *e = self.graph.start[u]);
            match self.dfs(source, sink, Flow::MAX) {
                Some(delta) => self.graph.excesses[sink] += delta,
                None => break,
            }
        }

        self.graph.excesses[sink]
    }

    fn dfs(&mut self, u: usize, sink: usize, upper: Flow) -> Option<Flow> {
        if u == sink {
            return Some(upper);
        }

        let mut res = 0 as Flow;
        for i in self.current_edge[u]..self.graph.start[u + 1] {
            self.current_edge[u] = i;
            let v = self.graph.inside_edge_list[i].to;
            let r = self.graph.inside_edge_list[i].residual_capacity();

            // check u -> v
            if !self.graph.is_admissible_edge(u, i) {
                continue;
            }

            match self.dfs(v, sink, r.min(upper - res)) {
                Some(d) => {
                    // update flow
                    let rev = self.graph.inside_edge_list[i].rev;
                    self.graph.inside_edge_list[i].flow += d;
                    self.graph.inside_edge_list[rev].flow -= d;

                    res += d;
                    if res == upper {
                        return Some(res);
                    }
                }
                None => continue,
            }
        }
        self.current_edge[u] = self.graph.start[u + 1];
        self.graph.distances[u] = self.graph.num_nodes();

        Some(res)
    }
}