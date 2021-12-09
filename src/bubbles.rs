use handlegraph::handle::{Direction, Handle};
use handlegraph::handlegraph::HandleGraph;
use handlegraph::hashgraph::HashGraph;
use itertools::Itertools;
use std::collections::VecDeque;

pub enum BubbleType {
    Simple,
    Complex,
}

pub struct Bubble {
    start: u64,
    end: u64,
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
    let bubbles: Vec<Bubble> = vec![];

    // Run a BFS visit of the graph, using a Queue
    let handles: Vec<Handle> = graph.handles_iter().sorted().collect();
    //let mut curr_width_queue: VecDeque<Handle> = VecDeque::new();
    //let mut next_width_queue: VecDeque<Handle> = VecDeque::new();
    let mut bfs_tree_widths: Vec<usize> = vec![];

    if !handles.is_empty() {
        // We know that the graph starts with a single node
        bfs_tree_widths.push(1);

        // Initialize the BFS
        let first_handle = handles.first().unwrap().clone();
        let mut new_front = advance_bfs(graph, vec![first_handle]);
        bfs_tree_widths.push(new_front.len());

        while !new_front.is_empty() {
            new_front = advance_bfs(graph, new_front);
            if !new_front.is_empty() {
                bfs_tree_widths.push(new_front.len());
            }
        }

        println!("Bfs tree widths are: {:#?}", bfs_tree_widths);

        /*
        loop {
            curr_width_queue = next_width_queue.clone();
            // Iterate over BFS tree
            while let Some(curr_handle) = curr_width_queue.pop_front() {
                // Check right edges in the original graph
                for handle in graph.handle_edges_iter(curr_handle, Direction::Right) {
                    if !next_width_queue.contains(&handle)
                        && handle.as_integer() > curr_handle.as_integer()
                    {
                        next_width_queue.push_back(handle);
                    }
                }
            }

            next_width_queue = curr_width_queue.clone();
        }
         */
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
    ///          | 2: AG \
    /// REV  1: T         4: TGC
    ///          \ 3: TC |
    fn create_simple_graph() -> HashGraph {
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

    #[test]
    fn test() {
        let graph = create_simple_graph();
        find_bubbles(&graph);
    }
}


