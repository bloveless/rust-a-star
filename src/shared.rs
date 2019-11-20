use std::collections::HashMap;

#[derive(PartialEq)]
pub enum CellType {
    Path = 0,
    Wall = 1,
    Node = 2,
}

pub struct Maze {
    pub height: u32,
    pub width: u32,
    pub cells: Vec<CellType>,
}

pub struct Graph {
    pub terminal_nodes: Vec<(u32, u32)>,
    pub nodes: HashMap<(u32, u32), GraphNode>,
}

pub struct GraphNode {
    pub x: u32,
    pub y: u32,
    pub visited: bool,
    pub relations: Vec<(u32, u32)>,
}

