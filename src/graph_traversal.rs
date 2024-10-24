/*
This code implements BFS, DFS, and NextSmallest node iteration on a Graph.
The iteration starts from a given node and then tries to visit all the nodes using the specific order.

NextSmallest always visits the next smallest item among the current seen nodes.

*/

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::iter::Iterator;

// Define a graph node structure
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Node {
    value: i32,
    neighbors: Vec<usize>, // List of indices representing neighbors
}

// Define a graph structure
#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
}

// BFS iterator for the graph
struct BfsIterator<'a> {
    graph: &'a Graph,
    queue: VecDeque<usize>,
    visited: HashSet<usize>,
}

impl<'a> BfsIterator<'a> {
    // Create a new BFS iterator starting from a given node index
    fn new(graph: &'a Graph, start: usize) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(start);

        Self {
            graph,
            queue,
            visited: HashSet::new(),
        }
    }
}

impl<'a> Iterator for BfsIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node_index) = self.queue.pop_front() {
            // If the node has already been visited, skip it
            if !self.visited.insert(node_index) {
                continue;
            }

            let node = &self.graph.nodes[node_index];

            // Add all unvisited neighbors to the queue
            for &neighbor_index in &node.neighbors {
                if !self.visited.contains(&neighbor_index) {
                    self.queue.push_back(neighbor_index);
                }
            }

            // Return the current node
            return Some(node);
        }
        None
    }
}

// DFS iterator for the graph
struct DfsIterator<'a> {
    graph: &'a Graph,
    stack: Vec<usize>,
    visited: HashSet<usize>,
}

impl<'a> DfsIterator<'a> {
    // Create a new DFS iterator starting from a given node index
    fn new(graph: &'a Graph, start: usize) -> Self {
        let mut stack = Vec::new();
        stack.push(start);

        Self {
            graph,
            stack,
            visited: HashSet::new(),
        }
    }
}

impl<'a> Iterator for DfsIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node_index) = self.stack.pop() {
            // If the node has already been visited, skip it
            if !self.visited.insert(node_index) {
                continue;
            }

            let node = &self.graph.nodes[node_index];

            // Add all unvisited neighbors to the stack (in reverse order)
            for &neighbor_index in node.neighbors.iter().rev() {
                if !self.visited.contains(&neighbor_index) {
                    self.stack.push(neighbor_index);
                }
            }

            // Return the current node
            return Some(node);
        }
        None
    }
}

// Next smallest node iterator for the graph
struct NextSmallestIterator<'a> {
    graph: &'a Graph,
    heap: BinaryHeap<Reverse<(i32, usize)>>, // (node value, node index)
    visited: HashSet<usize>,
}

impl<'a> NextSmallestIterator<'a> {
    // Create a new iterator starting from a given node index
    fn new(graph: &'a Graph, start: usize) -> Self {
        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::new();

        // Push the starting node into the heap
        if let Some(start_node) = graph.nodes.get(start) {
            heap.push(Reverse((start_node.value, start)));
            visited.insert(start);
        }

        Self {
            graph,
            heap,
            visited,
        }
    }
}

impl<'a> Iterator for NextSmallestIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(Reverse((_, node_index))) = self.heap.pop() {
            // Get the current node
            let node = &self.graph.nodes[node_index];

            // Add all unvisited neighbors to the heap
            for &neighbor_index in &node.neighbors {
                if !self.visited.contains(&neighbor_index) {
                    self.visited.insert(neighbor_index);
                    let neighbor_node = &self.graph.nodes[neighbor_index];
                    self.heap
                        .push(Reverse((neighbor_node.value, neighbor_index)));
                }
            }

            // Return the current node
            return Some(node);
        }
        None
    }
}

fn main() {
    // Create a simple graph
    let graph = Graph {
        nodes: vec![
            Node {
                value: 0,
                neighbors: vec![1, 2],
            }, // Node 0
            Node {
                value: 1,
                neighbors: vec![0, 5, 4],
            }, // Node 1
            Node {
                value: 2,
                neighbors: vec![0, 3],
            }, // Node 2
            Node {
                value: 3,
                neighbors: vec![1],
            }, // Node 3
            Node {
                value: 4,
                neighbors: vec![1, 2],
            }, // Node 4
            Node {
                value: 5,
                neighbors: vec![1, 2],
            }, // Node 4
        ],
    };

    // Create a BFS iterator starting from node 0
    let bfs_iter = BfsIterator::new(&graph, 0);

    // Iterate over the graph using BFS
    println!("BFS");
    for node in bfs_iter {
        println!("Visited node with value: {}", node.value);
    }

    // Create a DFS iterator starting from node 0
    let dfs_iter = DfsIterator::new(&graph, 0);

    // Iterate over the graph using BFS
    println!("DFS");
    for node in dfs_iter {
        println!("Visited node with value: {}", node.value);
    }

    // Create a NextSmallest iterator starting from node 0
    let ns_iter = NextSmallestIterator::new(&graph, 0);

    // Iterate over the graph using BFS
    println!("NextSmallest");
    for node in ns_iter {
        println!("Visited node with value: {}", node.value);
    }
}
