use super::shared;
use std::collections::HashMap;

pub fn run(graph: &shared::Graph) {
    let mut stack: Vec<(u32, u32)> = Vec::new();
    let mut visited: HashMap<(u32, u32), bool> = HashMap::new();

    if super::DEBUG {
        println!("Starting breadth first search");
    }

    let start_coords = graph.terminal_nodes.get(0).unwrap();

    if super::DEBUG {
        println!("Start node (x: {} y: {})", start_coords.0, start_coords.1);
    }

    stack.push(*start_coords);

    process(graph, &mut stack, &mut visited);
}

fn process(
    graph: &shared::Graph,
    stack: &mut Vec<(u32, u32)>,
    visited: &mut HashMap<(u32, u32), bool>,
) {
    while !stack.is_empty() {
        let cur_coords = stack.pop().unwrap();
        let end_coords = graph.terminal_nodes.get(1).unwrap();

        if cur_coords == *end_coords {
            println!("Found end");
        }

        if visited.contains_key(&cur_coords) {
            continue;
        }

        let cur_node = graph.nodes.get(&cur_coords).unwrap();
        visited.insert(cur_coords, true);

        for relation in cur_node.relations.iter() {
            if super::DEBUG {
                println!(
                    "Adding nodes to stack (x: {} y: {})",
                    relation.0, relation.1
                );
            }
            stack.push(*relation);
        }
    }

    if super::DEBUG {
        println!(
            "Reached the end of the graph. Visited {} nodes of {} total nodes.",
            visited.len(),
            graph.nodes.len()
        );
    }
}
