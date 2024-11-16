#![allow(clippy::needless_return)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::ops::Index;
use std::path::Path;
use itertools::Itertools;
use rand::Rng;
use num_format::Locale::{se, sr};
use serde_with::serde_as;
use serde_with::serde_derive::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub node: String,
    pub contractions: Vec<(String, String)>
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct GraphWrapper {
    #[serde_as(as = "Vec<(_, _)>")]
    x: HashMap::<String, Vec<GraphNode>>,
}

impl PartialEq<Self> for GraphNode {
    fn eq(&self, other: &Self) -> bool {
        return self.node == other.node;
    }
}

impl Eq for GraphNode {

}

fn main() {
    let path = Path::new("src/day25/test_input.txt");
    let data = parse_data(&path);

    let mut graph: HashMap::<String, Vec<GraphNode>> = HashMap::<String, Vec<GraphNode>>::new();
    for (src, connections) in data {
        graph.insert(src.clone(), Vec::<GraphNode>::new());

        for connection in connections {
            graph.get_mut(&src).unwrap().push(GraphNode {
                node: connection,
                contractions: Vec::<(String, String)>::new()
            });
        }
    }

    for _ in 0usize..1 {
        let mut fresh_graph = graph.clone();
        reduce_map(&mut fresh_graph);
        println!("Results: {:?}", fresh_graph.keys().collect_vec());

        let mut result_file = File::create(r#"D:\Users\Nicolas\Documents\RustProjects\advent-of-code-2023\src\day25\results.txt"#).unwrap();
        result_file.write(serde_json::to_string_pretty(&fresh_graph).unwrap().as_ref()).expect("Couldn't write results to file");
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

//Contract vertex 2 into vertex 1
fn contract(vertex1: &String, vertex2: &String, graph: &mut HashMap<String, Vec<GraphNode>>) {
    //Pull vertex 2 into vertex 1
    let mut vertex1_nodes = graph.remove(vertex1).unwrap();
    let mut vertex2_nodes = graph.remove(vertex2).unwrap();

    vertex2_nodes.remove(vertex2_nodes.iter().position(|x| *x.node == *vertex1).unwrap());

    for node in &mut vertex2_nodes {
        node.contractions.push((vertex2.clone(), node.node.clone()));
    }

    vertex1_nodes.extend(vertex2_nodes);
    vertex1_nodes = vertex1_nodes.clone().into_iter().unique().collect_vec();

    for graph_node in graph.values_mut() {
        for node in graph_node {
            if node.node == *vertex2 {
                node.node = vertex1.to_string();
            }
        }
    }

    graph.insert(vertex1.clone(), vertex1_nodes);

    //After update, update reference to vertex 2 and remove any self-references
    for entry in &mut *graph {
        for index in (0usize..entry.1.len()).into_iter().rev() {
            if entry.1[index].node == *vertex2 {
                entry.1[index].node = vertex1.clone();
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

fn reduce_map(graph: &mut HashMap<String, Vec<GraphNode>>) {
    let mut rng = rand::thread_rng();

    while graph.keys().len() > 2 {
        let key_index = rng.gen_range(0usize..graph.keys().len());
        let key = graph.keys().collect_vec()[key_index].clone();
        let value_index = rng.gen_range(0usize..graph[&key].len());
        let value = graph[&key][value_index].node.clone();

        println!("Contracting {value} into {key}");
        contract(&key, &value, graph);

        for entry in &mut *graph {
            if entry.1.iter().any(|x| x.node == *entry.0){
                println!("common entry: {}", *entry.0);
                let foo = 1;
            }
        }
    }
}