#![allow(clippy::needless_return)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use itertools::Itertools;
use num_format::Locale::it;
use rand::Rng;
use serde_with::serde_derive::{Deserialize, Serialize};

#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub node: String,
    pub original_node: String,
    pub contracted: bool,
    pub contractions: Vec<UnorderedPair<String>>
}

impl PartialEq<Self> for GraphNode {
    fn eq(&self, other: &Self) -> bool {
        return self.node == other.node;
    }
}

impl Eq for GraphNode {

}

#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
struct UnorderedPair<T> {
    left: T,
    right: T
}

impl<T: PartialEq> PartialEq<Self> for UnorderedPair<T> {
    fn eq(&self, other: &Self) -> bool {
        return (self.left == other.left && self.right == other.right) ||
            (self.left == other.right && self.right == other.left);
    }
}

impl<T: PartialEq> Eq for UnorderedPair<T>{

}

fn main() {
    let path = Path::new("src/day25/test_input.txt");
    let data = parse_data(&path);

    let mut graph: HashMap::<String, Vec<GraphNode>> = HashMap::<String, Vec<GraphNode>>::new();
    for (src, connections) in data {
        if !graph.contains_key(&src) {
            graph.insert(src.clone(), Vec::<GraphNode>::new());
        }

        for connection in connections {
            let node_list = graph.get_mut(&src).unwrap();
            if node_list.iter().all(|x| x.node != connection) {
                node_list.push(GraphNode {
                    node: connection.clone(),
                    original_node: connection.clone(),
                    contracted: false,
                    contractions: Vec::<UnorderedPair<String>>::new()
                });
            }

            //Add the complement mapping
            if !graph.contains_key(&connection) {
                graph.insert(connection.clone(), Vec::<GraphNode>::new());
            }

            let node_list = graph.get_mut(&connection).unwrap();
            if node_list.iter().all(|x| x.node != src) {
                node_list.push(GraphNode {
                    node: src.clone(),
                    original_node: src.clone(),
                    contracted: false,
                    contractions: Vec::<UnorderedPair<String>>::new()
                });
            }
        }
    }

    let mut graph_file = File::create(r#"D:\Users\Nicolas\Documents\RustProjects\advent-of-code-2023\src\day25\graph.txt"#).unwrap();
    graph_file.write(serde_json::to_string_pretty(&graph).unwrap().replace("\n", "\r\n").as_ref());

    let num_cuts = 3usize;
    let max_iterations = 10_000usize;
    let mut found = false;
    let mut optimal_min_graph = graph.clone();

    for _ in 0usize..max_iterations {
        let mut reduced_graph = graph.clone();
        reduce_map(&mut reduced_graph);

        if reduced_graph.values().collect_vec()[0][0].contractions.len() == num_cuts {
            found = true;
            optimal_min_graph = reduced_graph;
            break;
        }
    }

    let mut result_file = File::create(r#"D:\Users\Nicolas\Documents\RustProjects\advent-of-code-2023\src\day25\results.txt"#).unwrap();
    result_file.write(serde_json::to_string_pretty(&optimal_min_graph).unwrap().replace("\n", "\r\n").as_ref());
    result_file.flush();

    /*
    let a = "a".to_string();
    let b = "b".to_string();
    let c = "c".to_string();
    let d = "d".to_string();
    let e = "e".to_string();
    let f = "f".to_string();
    let g = "g".to_string();
    let h = "h".to_string();
    let i = "i".to_string();

    contract(&c, &a, &a, &mut graph);
    contract(&c, &b, &b, &mut graph);
    contract(&g, &e, &e, &mut graph);
    contract(&g, &f, &f, &mut graph);
    contract(&d, &g, &g, &mut graph);
    contract(&d, &i, &i, &mut graph);
    contract(&c, &h, &h, &mut graph);

    println!("Results: {:?}", graph.keys().collect_vec());
    let mut result_file = File::create(r#"D:\Users\Nicolas\Documents\RustProjects\advent-of-code-2023\src\day25\results.txt"#).unwrap();
    result_file.write(serde_json::to_string_pretty(&graph).unwrap().replace("\n", "\r\n").as_ref());
    result_file.flush();
     */

    if !found {
        println!("Solution not found");
    }
}

fn parse_data(path: &Path) -> HashMap<String, HashSet<String>> {
    let file = File::open(&path).unwrap();

    let line_splits = BufReader::new(file)
        .lines()
        .flatten()
        .map(|line| line.split(":").map(|s| s.trim().to_string()).collect_vec())
        .collect_vec();

    let keys = line_splits
        .iter()
        .map(|line_split| line_split[0].clone())
        .collect_vec();

    let values = line_splits
        .iter()
        .map(|line_split| HashSet::<String>::from_iter(line_split[1].split(" ").map(|s| s.to_string())))
        .collect_vec();

    let mut graph = HashMap::<String, HashSet<String>>::new();
    for key in &keys {
        graph.insert(key.clone(), HashSet::<String>::new());
    }

    for (src, connections) in keys.into_iter().zip(values.into_iter()) {
        for connection in &connections {
            graph.get_mut(&src.clone()).unwrap().insert(connection.clone());
            if !graph.contains_key(connection) {
                graph.insert(connection.clone(), HashSet::<String>::new());
            }

            graph.get_mut(connection).unwrap().insert(src.clone());
        }
    }

    return graph;
}

fn reduce_map(graph: &mut HashMap<String, Vec<GraphNode>>) {
    let mut rng = rand::thread_rng();
    let mut counter = 0usize;

    while graph.keys().len() > 2 {
        let key_index = rng.gen_range(0usize..graph.keys().len());
        let key = graph.keys().collect_vec()[key_index].clone();
        let value_index = rng.gen_range(0usize..graph[&key].len());
        let value = graph[&key][value_index].node.clone();
        let value_original_node = &graph[&key][value_index].original_node.clone();

        //println!("Contracting {value} into {key}");
        contract(&key, &value, value_original_node, graph);

        //let mut result_file = File::create(r#"D:\Users\Nicolas\Documents\RustProjects\advent-of-code-2023\src\day25\results.txt"#).unwrap();
        //result_file.write(serde_json::to_string_pretty(&graph).unwrap().replace("\n", "\r\n").as_ref());
        //result_file.flush();
        //println!("Counter = {counter}");
        counter += 1;
    }
}

//Contract vertex 2 into vertex 1
fn contract(vertex1: &String, vertex2: &String, vertex2_original_node: &String, graph: &mut HashMap<String, Vec<GraphNode>>) {
    //Remove vertex 2 nodes and save a record of the contracted nodes
    let vertex1_nodes = graph.remove(vertex1).unwrap();
    let mut vertex2_nodes = graph.remove(vertex2).unwrap();

    for node in vertex2_nodes.iter_mut().filter(|x| !x.contracted && x.node != *vertex1) {
        node.contractions.push(UnorderedPair {
            left: vertex2_original_node.clone(),
            right: node.original_node.clone()
        });
        node.contracted = true;

        let mut complement_node_connections = graph.get_mut(&node.node).unwrap();
        let index = complement_node_connections.iter().position(|x| x.node == *vertex2).unwrap();
        complement_node_connections[index].contractions.push(UnorderedPair {
            left: vertex2_original_node.clone(),
            right: node.original_node.clone()
        });
        complement_node_connections[index].contracted = true;
    }

    //Merge vertex 2 nodes into vertex 1
    let vertex1_nodes = merge_nodes(vertex1_nodes, vertex2_nodes);
    graph.insert(vertex1.clone(), vertex1_nodes);

    //After update, update references to vertex 2 and then remove any self-references
    for entry in &mut *graph {
        let mut vertex_updated = false;

        for index in (0usize..entry.1.len()).into_iter().rev() {
            if entry.1[index].node == *vertex2 {
                entry.1[index].node = vertex1.clone();
                vertex_updated = true;
            }
        }

        //Merge duplicate nodes if any
        if vertex_updated {
            for index in (0usize..entry.1.len()).into_iter().rev() {
                if entry.1.iter().filter(|&x| x.node == entry.1[index].node).count() == 1 { continue; }

                if let Some(merge_index) = entry.1.iter().position(|x| x.node == entry.1[index].node) {
                    let mut merged_contractions = HashSet::<UnorderedPair<String>>::from_iter(entry.1[merge_index].contractions.clone());

                    for contraction in &entry.1[index].contractions {
                        merged_contractions.insert(contraction.clone());
                    }

                    entry.1[merge_index].contractions = merged_contractions.into_iter().collect_vec();
                    entry.1.remove(index);
                }
            }
        }
    }

    let keys = graph.keys().map(|x| (*x).clone()).collect_vec();
    for key in keys {
        for index in (0usize..graph[&key].len()).into_iter().rev() {
            if graph[&key][index].node == key {
                graph.get_mut(&key).unwrap().remove(index);
            }
        }
    }
}

fn merge_nodes(vertex1_nodes: Vec<GraphNode>, mut vertex2_nodes: Vec<GraphNode>) -> Vec<GraphNode> {
    let mut merged_nodes = Vec::<GraphNode>::from_iter(vertex1_nodes);

    for mut vertex2_node in vertex2_nodes {
        if let Some(merged_node_index) = merged_nodes.iter().position(|x| x.node == vertex2_node.node) {
            let mut merged_contractions = HashSet::<UnorderedPair<String>>::from_iter(merged_nodes[merged_node_index].contractions.clone());
            for contraction in vertex2_node.contractions {
                merged_contractions.insert(contraction);
            }
            merged_nodes[merged_node_index].contractions = merged_contractions.into_iter().collect_vec();
        } else {
            merged_nodes.push(vertex2_node);
        }
    }

    return merged_nodes;
}