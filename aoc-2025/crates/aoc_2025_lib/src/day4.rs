use std::fs;
struct XmasSearch {
    grid: Vec<Vec<char>>,
    row_size: usize,
    col_size: usize
}


#[derive(Debug)]
enum GridSearchDirections {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight
}

impl XmasSearch {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let row_size = (&grid).len();
        let col_size = (&grid)[0].len();
        XmasSearch {
            grid,
            row_size,
            col_size
        }
    }

    fn in_bound(&self, direction: &GridSearchDirections, row: usize, col: usize) -> bool {
        let mut row = row as isize;
        let mut col = col as isize;
    
        match direction {
            GridSearchDirections::Left => {
                col -= 3;
            },
            GridSearchDirections::Right => {
                col += 3;
            },
            GridSearchDirections::Up => {
                row -= 3;
            }
            GridSearchDirections::Down => {
                row += 3;
            }
            GridSearchDirections::UpLeft => {
                row -= 3;
                col -= 3;
            },
            GridSearchDirections::UpRight => {
                row -= 3;
                col += 3;
            },
            GridSearchDirections::DownLeft => {
                row += 3;
                col -= 3;
            },
            GridSearchDirections::DownRight => {
                row += 3;
                col += 3;
            }
        }

        return row < self.row_size as isize && row >= 0 && col < self.col_size as isize && col >= 0;
    }

    fn next_cell(&self, direction: &GridSearchDirections, row: usize, col: usize, distance: usize) -> (usize, usize) {
        match direction {
            GridSearchDirections::Left => return (row, col-distance),
            GridSearchDirections::Right => return (row, col+distance),
            GridSearchDirections::Up => return (row-distance, col),
            GridSearchDirections::Down => return (row+distance, col),
            GridSearchDirections::UpLeft => return (row-distance, col-distance),
            GridSearchDirections::UpRight => return (row-distance, col+distance),
            GridSearchDirections::DownLeft => return (row+distance, col-distance),
            GridSearchDirections::DownRight => return (row+distance, col+distance)
        }
    }
    fn explore_cell(&self, row: usize, col: usize) -> u64 {
        let mut xmas_count = 0;
        let search_array = vec!['X', 'M', 'A', 'S'];
        let search_directions = vec![
            GridSearchDirections::Left,
            GridSearchDirections::Right,
            GridSearchDirections::Up,
            GridSearchDirections::Down,
            GridSearchDirections::UpLeft,
            GridSearchDirections::UpRight,
            GridSearchDirections::DownLeft,
            GridSearchDirections::DownRight
        ];

        for direction in search_directions {
            if !self.in_bound(&direction, row, col) {
                continue;
            }

            let mut found = true;
            for idx in 0..search_array.len() {
                let (search_row, search_col) = self.next_cell(&direction, row, col, idx);
                if self.grid[search_row][search_col] != search_array[idx] {
                    found = false;
                    break;
                }
            }

            if(found) {
                xmas_count += 1;
            }
            
        }

        xmas_count
    }

    fn compute(&self) -> u64 {
        let mut xmas_count = 0;

        for row in 0..self.row_size {
            for col in 0..self.col_size {
                xmas_count += self.explore_cell(row, col);
            }
        }
        xmas_count
    }
}


fn ceres_search(file_path: String) -> u64 {
    let grid = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    
    let xmas_search = XmasSearch::new(grid);

    xmas_search.compute()
}

pub fn main(file_path: String) {
    let ans = ceres_search(file_path);

    println!("XMAS count: {ans}");
}
