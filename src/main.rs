extern crate termion;

use termion::{color};

use std::{thread, time};

///struct for the maze containing the array for the nodes and the barriers
struct Maze {
    count: u32,
    width: usize,
    height: usize,
    horizontal_weight: f32,     // probability for a horizontal barrier
    vertical_weight: f32,       // probability for a vertical barrier
    grid: [[[u32; 3]; 10]; 20], // [[[node, right, bottom], width ], height]
}

impl Maze {
    ///prints the maze to the console in unicode
    fn print_maze(&self) {
        let mut res = String::from("");
        for h in self.grid {
            res.push_str(&'\u{007C}'.to_string());
            for w in h {
                if w[0] == self.count {
                    res += &format!(
                        "{}{:02}{}",
                        color::Fg(color::Red),
                        &w[0],
                        color::Fg(color::Reset)
                    );
                } else if w[0] != 0 {
                    res += &format!("{}{:02}{}", color::Fg(color::LightYellow), &w[0], color::Fg(color::Reset));
                } else {
                    res += &format!("{}{:02}", color::Fg(color::Reset), &w[0]);
                }
                if w[1] == 1 {
                    res.push_str(&'\u{007C}'.to_string());
                } else {
                    res.push_str(&'\u{0020}'.to_string());
                }
            }
            res.push('\n');
            res.push_str(&'\u{02D1}'.to_string());
            for w in h {
                if w[2] == 1 {
                    res.push_str(&'\u{2015}'.to_string());
                    res.push_str(&'\u{2015}'.to_string());
                } else {
                    res.push_str(&'\u{0020}'.to_string());
                    res.push_str(&'\u{0020}'.to_string());
                }
                res.push_str(&'\u{02D1}'.to_string());
            }
            res.push('\n');
        }

        println!("{}", res);
    }

    ///fill the maze borders
    fn fill_borders(&mut self) {
        for h in 0..self.height {
            for w in 0..self.width {
                self.grid[h][w][2] = if rand::random::<f32>() < self.horizontal_weight {
                    1
                } else {
                    0
                };
                self.grid[h][w][1] = if rand::random::<f32>() < self.vertical_weight {
                    1
                } else {
                    0
                };
            }
        }
    }

    ///set one node in the first row to 1
    fn init_start(&mut self, pos: usize) {
        if self.count == 0 {
            self.grid[0][pos][0] = 1;
            self.count = 1;
        } else {
            panic!("already initialized");
        }
    }

    ///set a node at position pos to count + 1
    fn set_grid_point_to_incr_count(&mut self, pos: (usize, usize)) {
        //(height, width)
        self.grid[pos.0][pos.1][0] = self.count + 1;
    }

    ///set neighboring nodes of the grid to the next count if they are not yet set and if there is no barrier
    #[allow(clippy::collapsible_if)]
    fn step(&mut self) {
        for h in 0..self.height {
            for w in 0..self.width {
                if self.grid[h][w][0] == self.count {
                    if h + 1 != self.height {
                        //if statement for bound protection
                        //if the neighboring node is not yet set and there is no barrier to this node set it to the counter + 1
                        if self.grid[h + 1][w][0] == 0 && self.grid[h][w][2] == 0 {
                            self.set_grid_point_to_incr_count((h + 1, w))
                        }
                    }
                    if h != 0 {
                        if self.grid[h - 1][w][0] == 0 && self.grid[h - 1][w][2] == 0 {
                            self.set_grid_point_to_incr_count((h - 1, w))
                        }
                    }
                    if w != 0 {
                        if self.grid[h][w - 1][0] == 0 && self.grid[h][w - 1][1] == 0 {
                            self.set_grid_point_to_incr_count((h, w - 1))
                        }
                    }
                    if w + 1 != self.width {
                        if self.grid[h][w + 1][0] == 0 && self.grid[h][w][1] == 0 {
                            self.set_grid_point_to_incr_count((h, w + 1))
                        }
                    }
                }
            }
        }
        self.count += 1;
    }

    ///check if the maze was solved by looking if the last row has non zeros
    fn check_if_solved(&self) -> bool {
        for node in self.grid[self.height - 1] {
            if node[0] != 0 && node[2] == 0 {
                return true;
            }
        }
        false
    }

    ///put everything together and solve the maze
    fn solve_maze(&mut self) {
        for _i in 0..self.width * self.height {
            self.step();
            if self.check_if_solved() {
                self.print_maze();
                break;
            }
        }
    }
}

///default values for the maze
impl Default for Maze {
    #[inline]
    fn default() -> Maze {
        Maze {
            count: 0,
            width: 10,
            height: 20,
            horizontal_weight: 0.5,
            vertical_weight: 0.5,
            grid: [[[0; 3]; 10]; 20],
        }
    }
}

fn main() {

    let ten_millis = time::Duration::from_millis(600);
    thread::sleep(ten_millis);

    // loop {
    //     //create instance of Maze
    //     //-------------------------------------------------------------------------
    //     let mut maze: Maze = Maze {
    //         vertical_weight: 0.6,
    //         ..Default::default()
    //     };

    //     //fill the maze
    //     //-------------------------------------------------------------------------
    //     maze.fill_borders();

    //     // set starting position
    //     //-------------------------------------------------------------------------
    //     maze.init_start(maze.width / 2);

    //     //solve the maze
    //     //-------------------------------------------------------------------------
    //     maze.solve_maze();
    // }

    let mut maze: Maze = Maze {
        horizontal_weight: 0.4,
        vertical_weight: 0.6,
        ..Default::default()
    };

    maze.fill_borders();

    maze.init_start(maze.width / 2);

    loop {
        maze.step();
        maze.print_maze();
        let ten_millis = time::Duration::from_millis(300);

        thread::sleep(ten_millis);
        print!("{esc}c", esc = 27 as char);
        if maze.check_if_solved() {
            maze.print_maze();
            break;
        }
    }
}
