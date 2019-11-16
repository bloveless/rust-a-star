// Mazes from: http://hereandabove.com/maze/mazeorig.form.html
extern crate image;

use std::env;
use std::time::Instant;
use std::path::Path;
use image::{GenericImageView, Rgba, GenericImage};

#[derive(PartialEq)]
enum CellType {
    Path = 0,
    Wall = 1,
    Node = 2,
}

struct Maze {
    height: u32,
    width: u32,
    cells: Vec<CellType>,
}

/**
 * Image requirements. One pixel for each wall or path. Black pixel for a wall and white pixel for a
 * path. The outermost pixel should represent the outermost wall with no padding.
 */

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    let mut img = image::open(&Path::new(&args[1])).unwrap();
    let (img_width, img_height) = img.dimensions();
    let mut maze = Maze {
        width: img_width,
        height: img_height,
        cells: Vec::new(),
    };

    for (_x, _y, pixel) in img.pixels() {
        if pixel[0] > 125 {
            maze.cells.push(CellType::Path);
        } else {
            maze.cells.push(CellType::Wall);
        }
    }

    find_nodes(&mut maze);
    find_entrance_and_exit(&maze);

    println!("Total run time: {}us", start_time.elapsed().as_micros());

    for y in 0..maze.height {
        for x in 0..maze.width {
            if maze.cells[coords_to_index(maze.width, x, y)] == CellType::Path {
                img.put_pixel(x, y, Rgba([255 as u8, 255 as u8, 255 as u8, 255 as u8]));
            }

            if maze.cells[coords_to_index(maze.width, x, y)] == CellType::Wall {
                img.put_pixel(x, y, Rgba([0 as u8, 0 as u8, 0 as u8, 255 as u8]));
            }

            if maze.cells[coords_to_index(maze.width, x, y)] == CellType::Node {
                img.put_pixel(x, y, Rgba([255 as u8, 0 as u8, 0 as u8, 255 as u8]));
            }
        }
    }

    img.save("output.png").unwrap();
}

fn find_nodes(maze: &mut Maze) {
    for y in 0..maze.height {
        for x in 0..maze.width {
            // check top, right, bottom, left and see if this is a node.
            // I.E. there are three paths connecting to it, or it creates a corner.
            // If there is only one path then it is an end node. If there are three or four paths
            // then it is a node. If there are two paths and they aren't directly across from each
            // other then it is a node.

            // Count paths.
            if maze.cells[coords_to_index(maze.width, x, y)] != CellType::Wall {
                let mut path_count: i8 = 0;

                // top
                if y > 0 && maze.cells[coords_to_index(maze.width, x, y - 1)] != CellType::Wall {
                    path_count += 1;
                }

                // right
                if x + 1 < maze.width && maze.cells[coords_to_index(maze.width, x + 1, y)] != CellType::Wall {
                    path_count += 1;
                }

                // bottom
                if y + 1 < maze.height && maze.cells[coords_to_index(maze.width, x, y + 1)] != CellType::Wall {
                    path_count += 1;
                }

                // left
                if x > 0 && maze.cells[coords_to_index(maze.width, x - 1, y)] != CellType::Wall {
                    path_count += 1;
                }

                if path_count == 1 {
                    let index = coords_to_index(maze.width, x, y);
                    maze.cells[index] = CellType::Node;
                }

                if path_count == 2 {
                    // if the two paths are across from each other then this is NOT a node
                    if x < maze.width
                        && maze.cells[coords_to_index(maze.width, x + 1, y)] != CellType::Wall
                        && x > 0
                        && maze.cells[coords_to_index(maze.width, x - 1, y)] != CellType::Wall {
                        continue;
                    }

                    if y < maze.width
                        && maze.cells[coords_to_index(maze.width, x, y + 1)] != CellType::Wall
                        && y > 0
                        && maze.cells[coords_to_index(maze.width, x, y - 1)] != CellType::Wall {
                        continue;
                    }

                    maze.cells[coords_to_index(maze.width, x, y)] = CellType::Node;
                }

                if path_count > 2 {
                    maze.cells[coords_to_index(maze.width, x, y)] = CellType::Node;
                }
            }
        }
    }
}

fn find_entrance_and_exit(maze: &Maze) {
    check_top_row_for_opening(maze);
    check_right_row_for_opening(maze);
    check_bottom_row_for_opening(maze);
    check_left_row_for_opening(maze);
}

fn check_top_row_for_opening(maze: &Maze) {
    for i in 0..maze.width {
        if maze.cells[coords_to_index(maze.width, i, 0)] == CellType::Path {
            println!("Found opening at (x: {}, y: {})", i, 0);
        }
    }
}

fn check_right_row_for_opening(maze: &Maze) {
    for i in 0..maze.height {
        if maze.cells[coords_to_index(maze.width, maze.width - 1, i)] == CellType::Path {
            println!("Found opening at (x: {}, y: {})", maze.width - 1, i);
        }
    }
}

fn check_bottom_row_for_opening(maze: &Maze) {
    for i in 0..maze.width {
        if maze.cells[coords_to_index(maze.width, i, maze.height - 1)] == CellType::Path {
            println!("Found opening at (x: {}, y: {})", i, maze.height - 1);
        }
    }
}

fn check_left_row_for_opening(maze: &Maze) {
    for i in 0..maze.height {
        if maze.cells[coords_to_index(maze.width, 0, i)] == CellType::Path {
            println!("Found opening at (x: {}, y: {})", 0, i);
        }
    }
}

fn coords_to_index(width: u32, x: u32, y: u32) -> usize {
    (width * y + x) as usize
}
