// Mazes from: http://hereandabove.com/maze/mazeorig.form.html
extern crate image;

mod shared;

mod breadth_first_search;

const DEBUG: bool = true;

use std::env;
use std::collections::HashMap;
use std::time::Instant;
use std::path::Path;
use image::{GenericImageView, Rgba, GenericImage, DynamicImage};

/**
 * Image requirements. One pixel for each wall or path. Black pixel for a wall and white pixel for a
 * path. The outermost pixel should represent the outermost wall with no padding.
 */

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    let mut img = image::open(&Path::new(&args[1])).unwrap();
    let (img_width, img_height) = img.dimensions();
    let mut maze = shared::Maze {
        width: img_width,
        height: img_height,
        cells: Vec::new(),
    };

    for (_x, _y, pixel) in img.pixels() {
        if pixel[0] > 125 {
            maze.cells.push(shared::CellType::Path);
        } else {
            maze.cells.push(shared::CellType::Wall);
        }
    }

    let start_find_nodes = Instant::now();
    find_nodes(&mut maze);
    if DEBUG {
        println!("Total time to find nodes: {}us", start_find_nodes.elapsed().as_micros());
    }

    let start_create_graph = Instant::now();
    let graph = create_graph(&maze);
    if DEBUG {
        println!("Total time to create the graph: {}us", start_create_graph.elapsed().as_micros());
    }

    if DEBUG {
        println!("Total solve time: {}us", start_time.elapsed().as_micros());
    }

    breadth_first_search::run(&graph);

    update_image_pixels(maze, &mut img);
    img.save("output.png").unwrap();
}

fn find_nodes(maze: &mut shared::Maze) {
    for y in 0..maze.height {
        for x in 0..maze.width {
            // Count paths.
            if maze.cells[coords_to_index(maze.width, x, y)] != shared::CellType::Wall {
                let mut path_count: i8 = 0;

                // top
                if y > 0 && maze.cells[coords_to_index(maze.width, x, y - 1)] != shared::CellType::Wall {
                    path_count += 1;
                }

                // right
                if x + 1 < maze.width && maze.cells[coords_to_index(maze.width, x + 1, y)] != shared::CellType::Wall {
                    path_count += 1;
                }

                // bottom
                if y + 1 < maze.height && maze.cells[coords_to_index(maze.width, x, y + 1)] != shared::CellType::Wall {
                    path_count += 1;
                }

                // left
                if x > 0 && maze.cells[coords_to_index(maze.width, x - 1, y)] != shared::CellType::Wall {
                    path_count += 1;
                }

                if path_count == 1 {
                    let index = coords_to_index(maze.width, x, y);
                    maze.cells[index] = shared::CellType::Node;
                }

                if path_count == 2 {
                    // if the two paths are across from each other then this is NOT a node
                    if x < maze.width
                        && maze.cells[coords_to_index(maze.width, x + 1, y)] != shared::CellType::Wall
                        && x > 0
                        && maze.cells[coords_to_index(maze.width, x - 1, y)] != shared::CellType::Wall {
                        continue;
                    }

                    if y < maze.width
                        && maze.cells[coords_to_index(maze.width, x, y + 1)] != shared::CellType::Wall
                        && y > 0
                        && maze.cells[coords_to_index(maze.width, x, y - 1)] != shared::CellType::Wall {
                        continue;
                    }

                    maze.cells[coords_to_index(maze.width, x, y)] = shared::CellType::Node;
                }

                if path_count > 2 {
                    maze.cells[coords_to_index(maze.width, x, y)] = shared::CellType::Node;
                }
            }
        }
    }
}

fn create_graph(maze: &shared::Maze) -> shared::Graph {
    let mut graph = shared::Graph {
        nodes: HashMap::new(),
        terminal_nodes: Vec::new(),
    };

    for y in 0..maze.height {
        for x in 0..maze.width {
            if maze.cells[coords_to_index(maze.width, x, y)] == shared::CellType::Node {
                let mut node = shared::GraphNode {
                    x,
                    y,
                    visited: false,
                    relations: Vec::new(),
                };

                join_cell_to_the_left(x, y, maze, &mut graph, &mut node);
                join_cell_to_the_top(x, y, maze, &mut graph, &mut node);

                graph.nodes.insert((x, y), node);

                if x == 0 || y == 0 || x == maze.width - 1 || y == maze.height - 1 {
                    let terminal_node = graph.nodes.get(&(x, y)).unwrap();
                    graph.terminal_nodes.push((x, y));
                }
            }
        }
    }

    graph
}

fn join_cell_to_the_left(x: u32, y: u32, maze: &shared::Maze, graph: &mut shared::Graph, node: &mut shared::GraphNode) {
    for cur_x in (0..x).rev() {
        match maze.cells[coords_to_index(maze.width, cur_x, y)] {
            shared::CellType::Wall => break,
            shared::CellType::Path => continue,
            shared::CellType::Node => {
                if DEBUG {
                    println!("Joining node at (x: {} y: {}) to node at (cur_x: {} y: {})", x, y, cur_x, y);
                }
                let other_node: &mut shared::GraphNode = match graph.nodes.get_mut(&(cur_x, y)) {
                    Some(node) => node,
                    None => panic!("Attempted to join on a node that didn't exist in the graph"),
                };
                other_node.relations.push((x, y));
                node.relations.push((cur_x, y));
                break;
            }
        }
    }
}

fn join_cell_to_the_top(x: u32, y: u32, maze: &shared::Maze, graph: &mut shared::Graph, node: &mut shared::GraphNode) {
    for cur_y in (0..y).rev() {
        match maze.cells[coords_to_index(maze.width, x, cur_y)] {
            shared::CellType::Wall => break,
            shared::CellType::Path => continue,
            shared::CellType::Node => {
                if DEBUG {
                    println!("Joining node at (x: {} y: {}) to node at (x: {} cur_y: {})", x, y, x, cur_y);
                }
                let other_node: &mut shared::GraphNode = match graph.nodes.get_mut(&(x, cur_y)) {
                    Some(node) => node,
                    None => panic!("Attempted to join on a node that didn't exist in the graph"),
                };
                other_node.relations.push((x, y));
                node.relations.push((x, cur_y));
                break;
            }
        }
    }
}

fn coords_to_index(width: u32, x: u32, y: u32) -> usize {
    (width * y + x) as usize
}

fn update_image_pixels(maze: shared::Maze, img: &mut DynamicImage) {
    for y in 0..maze.height {
        for x in 0..maze.width {
            if maze.cells[coords_to_index(maze.width, x, y)] == shared::CellType::Path {
                img.put_pixel(x, y, Rgba([255 as u8, 255 as u8, 255 as u8, 255 as u8]));
            }

            if maze.cells[coords_to_index(maze.width, x, y)] == shared::CellType::Wall {
                img.put_pixel(x, y, Rgba([0 as u8, 0 as u8, 0 as u8, 255 as u8]));
            }

            if maze.cells[coords_to_index(maze.width, x, y)] == shared::CellType::Node {
                img.put_pixel(x, y, Rgba([255 as u8, 0 as u8, 0 as u8, 255 as u8]));
            }
        }
    }
}