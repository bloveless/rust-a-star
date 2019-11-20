use super::shared;

pub fn run(graph: &shared::Graph) {
    let start_coords = graph.terminal_nodes.get(0).unwrap();
    let start_node = graph.nodes.get(start_coords).unwrap();
}
