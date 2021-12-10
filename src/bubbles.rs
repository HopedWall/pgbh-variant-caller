use handlegraph::handle::{Direction, Handle};
use handlegraph::handlegraph::HandleGraph;
use handlegraph::hashgraph::HashGraph;
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum BubbleType {
    Simple,
    Complex,
}

#[derive(Debug)]
pub struct Bubble {
    pub(crate) start: u64,
    pub(crate) end: u64,
    bubble_type: BubbleType,
}

// Only keep forward edges
fn find_neighbors_to_add(graph: &HashGraph, curr_handle: Handle) -> Vec<Handle> {
    graph
        .handle_edges_iter(curr_handle, Direction::Right)
        .filter_map(|x|
            match x.as_integer() > curr_handle.as_integer() {
                true => Some(x),
                _ => None
            })
        .collect()
}

// Advance the current front of the BFS
fn advance_bfs(graph: &HashGraph, curr_front: Vec<Handle>) -> Vec<Handle> {
    curr_front
        .iter()
        .flat_map(|x| find_neighbors_to_add(graph, *x))
        .sorted()
        .dedup()
        .collect()
}

pub fn find_bubbles(graph: &HashGraph) -> Vec<Bubble> {
    let mut bubbles: Vec<Bubble> = vec![];

    // Run a BFS visit of the graph, using a Queue
    let handles: Vec<Handle> = graph.handles_iter().sorted().collect();
    let mut bfs_tree_widths: Vec<usize> = vec![];
    let mut fronts: Vec<Vec<Handle>> = vec![];

    if !handles.is_empty() {
        // We know that the graph starts with a single node
        bfs_tree_widths.push(1);

        // Initialize the BFS
        let first_handle = handles.first().unwrap().clone();
        let first_front = vec![first_handle];
        fronts.push(first_front.clone());

        let mut new_front = advance_bfs(graph, first_front.clone());
        bfs_tree_widths.push(new_front.len());
        fronts.push(new_front.clone());

        while !new_front.is_empty() {
            new_front = advance_bfs(graph, new_front);
            if !new_front.is_empty() {
                fronts.push(new_front.clone());
                bfs_tree_widths.push(new_front.len());
            }
        }

        println!("Fronts are: {:#?}", fronts);
        println!("Bfs tree widths are: {:#?}", bfs_tree_widths);

        let mut bubble_open: VecDeque<Handle> = VecDeque::new();
        let mut bubble_close: VecDeque<Handle> = VecDeque::new();
        let mut curr_handle_id: u64 = 2;
        for level in 0..bfs_tree_widths.len() - 1 {
            let width_i = *bfs_tree_widths.get(level).unwrap();
            let width_j = *bfs_tree_widths.get(level+1).unwrap();

            if width_j > width_i {
                bubble_open.push_front(Handle::from_integer(curr_handle_id));
            } else if width_j < width_i {
                bubble_close.push_front(Handle::from_integer(curr_handle_id + 2*width_i as u64));
            }

            curr_handle_id += 2*width_i as u64;
        }

        println!("Bubble opens are: {:#?}", bubble_open);
        println!("Bubble closes are: {:#?}", bubble_close);

        //assert_eq!(bubble_open.len(), bubble_close.len());
        for (open, close) in bubble_open.iter().zip(bubble_close.iter()) {
            bubbles.push(Bubble {
                start: open.as_integer(),
                end: close.as_integer(),
                bubble_type: BubbleType::Simple
            });
        }

        println!("Bubbles: {:#?}", bubbles);

        for edge in graph.edges_iter() {
            println!("From {:#?} to {:#?}", edge.0.as_integer(), edge.1.as_integer());
        }

    }

    bubbles
}

#[cfg(test)]
mod test {
    use handlegraph::handle::Edge;
    use handlegraph::mutablehandlegraph::MutableHandleGraph;
    use handlegraph::pathgraph::PathHandleGraph;
    use super::*;

    /// This function creates a simple graph, used for debugging
    ///          | 2: CT \
    /// FWD  1: A         4: GCA
    ///          \ 3: GA |
    ///
    fn create_simple_graph_bubble() -> HashGraph {
        let mut graph: HashGraph = HashGraph::new();

        let h1 = graph.create_handle("A".as_bytes(), 1);
        let h2 = graph.create_handle("CT".as_bytes(), 2);
        let h3 = graph.create_handle("GA".as_bytes(), 3);
        let h4 = graph.create_handle("GCA".as_bytes(), 4);

        graph.create_edge(&Edge(h1, h2));
        graph.create_edge(&Edge(h1, h3));
        graph.create_edge(&Edge(h2, h4));
        graph.create_edge(&Edge(h3, h4));

        let p1 = graph.create_path_handle("P1".as_bytes(), false);
        graph.append_step(&p1, h1);
        graph.append_step(&p1, h2);
        graph.append_step(&p1, h4);

        let p2 = graph.create_path_handle("P2".as_bytes(), false);
        graph.append_step(&p2, h1);
        graph.append_step(&p2, h3);
        graph.append_step(&p2, h4);

        graph
    }

    /// This function creates a simple graph, used for debugging
    fn create_simple_graph_superbubble() -> HashGraph {
        let mut graph: HashGraph = HashGraph::new();

        let h1 = graph.create_handle("AAA".as_bytes(), 1);
        let h2 = graph.create_handle("CT".as_bytes(), 2);
        let h3 = graph.create_handle("GA".as_bytes(), 3);
        let h4 = graph.create_handle("CC".as_bytes(), 4);
        let h5 = graph.create_handle("TTT".as_bytes(), 5);
        let h6 = graph.create_handle("A".as_bytes(), 6);
        let h7 = graph.create_handle("GCA".as_bytes(), 7);

        graph.create_edge(&Edge(h1, h2));
        graph.create_edge(&Edge(h1, h3));

        graph.create_edge(&Edge(h2, h4));
        graph.create_edge(&Edge(h2, h5));
        graph.create_edge(&Edge(h4, h6));
        graph.create_edge(&Edge(h5, h6));
        graph.create_edge(&Edge(h6, h7));

        graph.create_edge(&Edge(h3, h7));

        graph
    }

    #[test]
    fn test_bubble() {
        let graph = create_simple_graph_bubble();
        find_bubbles(&graph);
    }

    #[test]
    fn test_superbubble() {
        let graph = create_simple_graph_superbubble();
        find_bubbles(&graph);
    }
}


