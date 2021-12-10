mod bubbles;
mod io;

use std::fs;
use std::path::PathBuf;
use gfa::gfa::GFA;
use gfa::parser::GFAParser;
use handlegraph::handle::Edge;
use handlegraph::handlegraph::HandleGraph;
use handlegraph::hashgraph::HashGraph;
use crate::bubbles::{Bubble, find_bubbles};
use crate::io::{write_bubbles_to_file, write_edges_to_file};

fn main() {

    // Create HashGraph from GFA
    let parser = GFAParser::new();
    let graphs_folder = PathBuf::from(&"./graphs");

    let graph_id: u64 = 0;
    for entry in fs::read_dir(graphs_folder).unwrap() {

        let file = entry.unwrap().path();
        let dir = file.parent().unwrap().to_path_buf();

        // Build the graph
        let gfa: GFA<usize, ()> = parser
            .parse_file(&PathBuf::from(file))
            .unwrap();
        let graph = HashGraph::from_gfa(&gfa);

        // Obtain the file with the edges
        let edges: Vec<Edge> = graph.edges_iter().collect();
        write_edges_to_file(&graph_id, &edges, &dir.join("/edges.csv")).unwrap();

        // Obtain the files with the bubbles
        let bubbles: Vec<Bubble> = find_bubbles(&graph);
        write_bubbles_to_file(&graph_id, &bubbles, &dir.join("/bubbles.csv")).unwrap();
    }
}
