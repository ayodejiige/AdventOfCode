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

    fn compute_xmas_occurrence_cell(&self, row: usize, col: usize) -> u64 {
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

            if found {
                xmas_count += 1;
            }
            
        }

        xmas_count
    }

    fn compute_xmas_occurrence(&self) -> u64 {
        let mut xmas_count = 0;

        for row in 0..self.row_size {
            for col in 0..self.col_size {
                xmas_count += self.compute_xmas_occurrence_cell(row, col);
            }
        }
        xmas_count
    }

    /// Check if the current cell is the center of a X-MAS.
    fn is_x_mas_cell(&self, col: usize, row: usize) -> bool {
        // If cell is at the edge of the grid, there are not enough surrounding cells
        // to form a X. 
        if col == 0 || row == 0 || col == self.col_size-1 || row == self.row_size-1 {
            return false;
        }

        // If cell value is not 'A', it cannot be an X-MAS cell
        if self.grid[row][col] != 'A' {
            return false;
        }

        let top_left_char = self.grid[row-1][col-1];
        let top_right_char = self.grid[row-1][col+1];
        let bottom_left_char = self.grid[row+1][col-1];
        let bottom_right_char = self.grid[row+1][col+1];

        let right_diagonal_is_mas = (top_left_char == 'M' && bottom_right_char == 'S') 
            || (top_left_char == 'S' && bottom_right_char == 'M');
        let left_diagonal_is_mas = (top_right_char == 'M' && bottom_left_char == 'S')
            || (top_right_char == 'S' && bottom_left_char == 'M');

        right_diagonal_is_mas && left_diagonal_is_mas 
    }

    /// Compute number of x-mas occurrences i.e. parts in the grid
    /// where two diagonal instances of MAS interserct. This center of the
    /// X will be the letter 'A' and the edges 'M' or 'A'
    fn compute_x_mas_occurrence(&self) -> u64 {
        let mut x_mas_count = 0;
        
        for row in 0..self.row_size {
            for col in 0..self.col_size {
                if self.is_x_mas_cell(col, row) {
                    x_mas_count += 1;
                }
            }
        }
        x_mas_count
    }

}

pub fn main(file_path: String) {
    let grid = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    
    let xmas_search = XmasSearch::new(grid);

    let ans = xmas_search.compute_xmas_occurrence();
    println!("XMAS count: {ans}");

    let ans = xmas_search.compute_x_mas_occurrence();
    println!("X-MAS count: {ans}");
}
