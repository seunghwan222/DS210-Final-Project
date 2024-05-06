mod graph_construction;
mod centrality;



fn main() {
    let file_path = "/Users/seunghwan/Desktop/DS210/final_project/stock_analysis/src/all_stocks_5yr.csv";
    let graph_result = graph_construction::build_graph(file_path, 0.5);

    if let Ok(graph) = graph_result {
        let centrality_scores = centrality::calculate_betweenness_centrality(&graph);
        
        let mut sorted_scores: Vec<(String, f64)> = graph.node_indices()
            .map(|n| (graph.node_weight(n).unwrap().clone(), *centrality_scores.get(&n.index()).unwrap_or(&0.0)))
            .collect();
        
        sorted_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        for (stock, score) in sorted_scores {
            println!("Stock: {}, Centrality: {:.4}", stock, score);
        }
    } else {
        eprintln!("Failed to build graph: {:?}", graph_result.err().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use petgraph::graph::DiGraph;
    #[test]
    fn test_simple_graph_construction() {
   
        let mut graph = DiGraph::<String, f64>::new();
        let node_a = graph.add_node("AAPL".to_string());
        let node_b = graph.add_node("MSFT".to_string());
        graph.add_edge(node_a, node_b, 0.6); 

        assert_eq!(graph.node_count(), 2); 
        assert_eq!(graph.edge_count(), 1); 
    }
}


