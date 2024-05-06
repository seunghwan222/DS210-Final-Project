use petgraph::graph::DiGraph;
use petgraph::visit::{EdgeRef, IntoNodeIdentifiers, NodeIndexable};
use petgraph::Direction;
use std::collections::{HashMap, VecDeque};



pub fn calculate_betweenness_centrality(graph: &DiGraph<String, f64>) -> HashMap<usize, f64> {
    let mut centrality = HashMap::new();
    let mut stack = Vec::new();
    let mut sigma = vec![0; graph.node_count()];
    let mut distance = vec![-1; graph.node_count()];
    let mut queue = VecDeque::new();
    let mut predecessors = vec![Vec::new(); graph.node_count()];
    let mut dependency = vec![0.0; graph.node_count()];

    for s in graph.node_identifiers() {
        let s_index = graph.to_index(s);
        for node in graph.node_identifiers() {
            let index = graph.to_index(node);
            predecessors[index].clear();
            sigma[index] = 0;
            distance[index] = -1;
        }
        sigma[s_index] = 1;
        distance[s_index] = 0;
        queue.clear();
        queue.push_back(s);
        stack.clear();
        stack.push(s);

        while let Some(v) = queue.pop_front() {
            let v_index = graph.to_index(v);
            for edge in graph.edges_directed(v, Direction::Outgoing) {
                let w = edge.target();
                let w_index = graph.to_index(w);
                if distance[w_index] == -1 {
                    queue.push_back(w);
                    distance[w_index] = distance[v_index] + 1;
                    stack.push(w);
                }
                if distance[w_index] == distance[v_index] + 1 {
                    sigma[w_index] += sigma[v_index];
                    predecessors[w_index].push(v);
                }
            }
        }

        while let Some(w) = stack.pop() {
            let w_index = graph.to_index(w);
            for &v in &predecessors[w_index] {
                let v_index = graph.to_index(v);
                dependency[v_index] += (sigma[v_index] as f64 / sigma[w_index] as f64) * (1.0 + dependency[w_index]);
            }
            if w != s {
                *centrality.entry(w_index).or_insert(0.0) += dependency[w_index];
            }
            dependency[w_index] = 0.0;
        }
    }

    centrality
}
