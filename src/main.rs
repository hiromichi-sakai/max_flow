use std::env;
use std::time::Instant;

use bipartite_push_relabel_fifo::BipartitePushRelabelFIFO;
use dinic::Dinic;
use bipartite_push_relabel_highest_label::BipartitePushRelabelHighestLabel;
use graph::Graph;

mod graph;
mod bipartite_push_relabel_fifo;
mod bipartite_push_relabel_highest_label;
mod dinic;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut graph = Graph::new();
    let data = std::fs::read_to_string(file_path).unwrap();
    let data: Vec<&str> = data.trim().split('\n').collect();

    let (num_left_nodes, num_right_nodes) = {
        let header: Vec<&str> = data[0].trim().split(' ').collect();
        let left_nodes: usize = header[1].parse().unwrap();
        let right_nodes: usize = header[4].parse().unwrap();
        (left_nodes, right_nodes)
    };
    let source = {
        let header: Vec<&str> = data[2].trim().split(' ').collect();
        let a: usize = header[1].parse().unwrap();
        a
    };
    let sink = {
        let header: Vec<&str> = data[3].trim().split(' ').collect();
        let a: usize = header[1].parse().unwrap();
        a
    };

    for line in data[4..].iter() {
        let line: Vec<&str> = line.trim().split(' ').collect();
        let (from, to, capacity) = (line[0].parse().unwrap(), line[1].parse().unwrap(), line[2].parse().unwrap());
        graph.add_directed_edge(from, to, capacity);
    }

    let mut fi = BipartitePushRelabelFIFO::new(num_left_nodes, num_right_nodes, &mut graph);
    let start = Instant::now();
    let fi_ans = fi.solve(source, sink);
    let fi_time = start.elapsed();

    graph.clear();
    let mut hi = BipartitePushRelabelHighestLabel::new(num_left_nodes, num_right_nodes, &mut graph);
    let start = Instant::now();
    let hi_ans = hi.solve(source, sink);
    let hi_time = start.elapsed();

    graph.clear();
    let mut dinic = Dinic::new(&mut graph);
    let start = Instant::now();
    let dinic_ans = dinic.solve(source, sink);
    let dinic_time = start.elapsed();

    assert_eq!(fi_ans, hi_ans);
    assert_eq!(fi_ans, dinic_ans);
    let name = &file_path[7..file_path.len() - 3];
    println!("{},{},{},{}", name, fi_time.as_millis(), hi_time.as_millis(), dinic_time.as_millis());
}

