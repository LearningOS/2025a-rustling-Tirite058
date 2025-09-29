/*
	graph
	This problem requires you to implement a basic graph functio
*/
// I AM DONE

use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        //TODO
        let node_str = node.to_string();
        if self.contains(node) {
            false
        } else {
            self.adjacency_table_mutable().insert(node_str, Vec::new());
            true
        }
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
        let (from, to, weight) = edge;
        
        // 确保两个节点都存在
        if !self.contains(from) {
            self.add_node(from);
        }
        if !self.contains(to) {
            self.add_node(to);
        }
        
        // 获取邻接表的可变引用
        let adj_table = self.adjacency_table_mutable();
        
        // 添加从 from 到 to 的边
        adj_table.get_mut(from).unwrap().push((to.to_string(), weight));
        // 添加从 to 到 from 的边（无向图）
        adj_table.get_mut(to).unwrap().push((from.to_string(), weight));
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
        // 调用 trait 中的默认实现
        Graph::add_edge(self, edge)
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            ("a", "b", 5),
            ("b", "a", 5),
            ("c", "a", 7),
            ("a", "c", 7),
            ("b", "c", 10),
            ("c", "b", 10),
        ];
        let actual_edges = graph.edges();
        println!("Actual edges: {:?}", actual_edges);
        println!("Expected edges count: {}", expected_edges.len());
        println!("Actual edges count: {}", actual_edges.len());
        
        for expected in expected_edges.iter() {
            let found = actual_edges.iter()
                .any(|(f, t, w)| f.as_str() == expected.0 && t.as_str() == expected.1 && w == &expected.2);
            assert!(found, "Missing expected edge: {:?}", expected);
        }
    }
}