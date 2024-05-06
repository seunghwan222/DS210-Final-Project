use petgraph::graph::{DiGraph, NodeIndex};
use serde::Deserialize;
use csv::ReaderBuilder;
use std::fs::File;
use std::io::{self, BufReader};
use std::collections::HashMap;



#[derive(Debug, Deserialize)]
pub struct StockRecord {
    close: f64,
    #[serde(rename = "Name")]
    name: String,
}

pub fn calculate_pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
    let mean_x = x.iter().sum::<f64>() / x.len() as f64;
    let mean_y = y.iter().sum::<f64>() / y.len() as f64;
    let numerator: f64 = x.iter().zip(y.iter()).map(|(&xi, &yi)| (xi - mean_x) * (yi - mean_y)).sum();
    let denominator: f64 = (x.iter().map(|&xi| (xi - mean_x).powi(2)).sum::<f64>() * y.iter().map(|&yi| (yi - mean_y).powi(2)).sum::<f64>()).sqrt();
    if denominator == 0.0 {
        0.0  
    } else {
        numerator / denominator
    }
}

pub fn build_graph(file_path: &str, threshold: f64) -> io::Result<DiGraph<String, f64>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

    let mut graph = DiGraph::<String, f64>::new();
    let mut name_index_map: HashMap<String, NodeIndex> = HashMap::new();
    let mut stocks_data: HashMap<String, Vec<f64>> = HashMap::new();

    for result in csv_reader.deserialize::<StockRecord>() {
        let record = result?;
        stocks_data.entry(record.name.clone()).or_insert_with(Vec::new).push(record.close);
    }

    for name in stocks_data.keys() {
        name_index_map.entry(name.clone()).or_insert_with(|| graph.add_node(name.clone()));
    }

    for (name1, data1) in &stocks_data {
        for (name2, data2) in &stocks_data {
            if name1 != name2 {
                let correlation = calculate_pearson_correlation(data1, data2);
                if correlation.abs() > threshold {
                    let node1 = name_index_map[name1];
                    let node2 = name_index_map[name2];
                    graph.add_edge(node1, node2, correlation);
                    graph.add_edge(node2, node1, correlation);
                }
            }
        }
    }

    Ok(graph)
}
