mod bubbles;
mod io;

use crate::bubbles::{find_bubbles, Bubble};
use crate::io::{write_all_bubbles_to_file, write_all_edges_to_file};
use gfa::gfa::GFA;
use gfa::parser::GFAParser;
use handlegraph::handle::Edge;
use handlegraph::handlegraph::HandleGraph;
use handlegraph::hashgraph::HashGraph;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Create HashGraph from GFA
    let parser = GFAParser::new();
    let graphs_folder = PathBuf::from(&"./graphs");

    // For each graph, store an id, its edges and its bubbles
    let mut graph_id: u64 = 0;
    let mut edges_with_id: Vec<(u64, Edge)> = vec![];
    let mut bubbles_with_id: Vec<(u64, Bubble)> = vec![];

    for entry in fs::read_dir(graphs_folder.clone()).unwrap() {
        // Check each file
        let file = entry.unwrap().path();

        // If we have a graph (in .gfa format)
        if file.extension().unwrap_or_default() == "gfa" {
            // Build the graph
            let gfa: GFA<usize, ()> = parser.parse_file(&PathBuf::from(file)).unwrap();
            let graph = HashGraph::from_gfa(&gfa);

            // Find edges and bubbles for each graph
            let curr_edges: Vec<Edge> = graph.edges_iter().collect();
            let curr_bubbles: Vec<Bubble> = find_bubbles(&graph);

            // Extend the overall edges/bubbles
            edges_with_id.extend(curr_edges.into_iter().map(|x| (graph_id, x)));
            bubbles_with_id.extend(curr_bubbles.into_iter().map(|x| (graph_id, x)));

            graph_id += 1;
        }
    }

    // Store edges in "edges.csv" and bubbles in "bubbles.csv"
    write_all_edges_to_file(&edges_with_id, &graphs_folder.join("edges.csv")).unwrap();
    write_all_bubbles_to_file(&bubbles_with_id, &graphs_folder.join("bubbles.csv")).unwrap();
}
