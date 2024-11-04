#![allow(clippy::needless_return)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use itertools::Itertools;
use ndarray::s;
use num_format::Locale::{se, sr};

#[derive(Debug, Hash, Clone)]
pub struct GraphNode {
    pub Node: String,
    pub Contractions: Vec<(String, String)>
}

impl PartialEq<Self> for GraphNode {
    fn eq(&self, other: &Self) -> bool {
        return self.Node == other.Node;
    }
}

impl Eq for GraphNode {

}

fn main() {
    let path = Path::new("src/day25/test_input.txt");
    let mut data = parse_data(&path);

    let mut graph: HashMap::<String, Vec<GraphNode>> = HashMap::<String, Vec<GraphNode>>::new();
    for (src, connections) in data {
        graph.insert(src.clone(), Vec::<GraphNode>::new());

        for connection in connections {
            graph.get_mut(&src).unwrap().push(GraphNode {
                Node: connection,
                Contractions: Vec::<(String, String)>::new()
            });
        }
    }

    //println!("{:?}", data)
    println!("{:?}", graph);
    contract(&"xhk".to_string(), &"hfx".to_string(), &mut graph);
    println!("---------------------------------------");
    println!("{:?}", graph);
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

fn contract(vertex1: &String, vertex2: &String, graph: &mut HashMap<String, Vec<GraphNode>>) {
    //Pull vertex 2 into vertex 1
    let mut vertex1Nodes = graph.remove(vertex1).unwrap();
    let mut vertex2Nodes = graph.remove(vertex2).unwrap();

    for index in 0..vertex1Nodes.len() {
        if vertex1Nodes[index].Node == *vertex2 {
            vertex1Nodes.remove(index);
            break;
        }
    }

    for mut node in &mut vertex2Nodes {
        node.Contractions.push((vertex2.clone(), node.Node.clone()));
    }

    vertex1Nodes.extend(vertex2Nodes);
    vertex1Nodes = vertex1Nodes.clone().into_iter().unique().collect_vec();

    for graphNode in graph.values_mut() {
        for node in graphNode {
            if node.Node == *vertex2 {
                node.Node = vertex1.to_string();
            }
        }
    }

    graph.insert(vertex1.clone(), vertex1Nodes);
}