

pub mod maze_mod {
    use termion::color; // has to be in the module

    pub struct Maze {
        pub count: u32,
        pub width: usize,
        pub height: usize,
        pub horizontal_weight: f32,     // probability for a horizontal barrier
        pub vertical_weight: f32,       // probability for a vertical barrier
        pub grid: [[[u32; 3]; 20]; 20], // [[[node, right, bottom], width ], height]
    }

    impl Maze {
        ///prints the maze to the console in unicode
        pub fn print_maze(&self) {
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
                        res += &format!(
                            "{}{:02}{}",
                            color::Fg(color::LightYellow),
                            &w[0],
                            color::Fg(color::Reset)
                        );
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
        pub fn fill_borders(&mut self) {
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

        ///set one node at pos to 1 (width, height)
        pub fn init_start(&mut self, pos: (usize, usize)) {
            //(width, height)
            if self.count == 0 {
                self.grid[pos.1][pos.0][0] = 1;
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
        ///if no node is changed in one step it is not solveable
        #[allow(clippy::collapsible_if)]
        pub fn step(&mut self) -> bool {
            let mut stuck: bool = true;
            for h in 0..self.height {
                for w in 0..self.width {
                    if self.grid[h][w][0] == self.count {
                        if h + 1 != self.height {
                            //if statement for bound protection
                            //if the neighboring node is not yet set and there is no barrier to this node set it to the counter + 1
                            if self.grid[h + 1][w][0] == 0 && self.grid[h][w][2] == 0 {
                                self.set_grid_point_to_incr_count((h + 1, w));
                                stuck = false;
                            }
                        }
                        if h != 0 {
                            if self.grid[h - 1][w][0] == 0 && self.grid[h - 1][w][2] == 0 {
                                self.set_grid_point_to_incr_count((h - 1, w));
                                stuck = false;
                            }
                        }
                        if w != 0 {
                            if self.grid[h][w - 1][0] == 0 && self.grid[h][w - 1][1] == 0 {
                                self.set_grid_point_to_incr_count((h, w - 1));
                                stuck = false;
                            }
                        }
                        if w + 1 != self.width {
                            if self.grid[h][w + 1][0] == 0 && self.grid[h][w][1] == 0 {
                                self.set_grid_point_to_incr_count((h, w + 1));
                                stuck = false;
                            }
                        }
                    }
                }
            }
            self.count += 1;
            stuck
        }

        ///check if the maze was solved by looking if the last row has non zeros
        pub fn check_if_solved(&self) -> bool {
            for node in self.grid[self.height - 1] {
                if node[0] != 0 && node[2] == 0 {
                    return true;
                }
            }
            false
        }

        ///put everything together and solve the maze
        pub fn solve_maze(&mut self) {
            for _i in 0..self.width * self.height {
                if self.step() {
                    break;
                }
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
                width: 20,
                height: 20,
                horizontal_weight: 0.5,
                vertical_weight: 0.5,
                grid: [[[0; 3]; 20]; 20],
            }
        }
    }
}
