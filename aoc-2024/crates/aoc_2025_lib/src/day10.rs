use std::collections::HashSet;
use std::fs;

/// Struct representing a topographical map.
struct TopographicalMap {
    /// 2D vector representing the topographical map. The value at each cell represents the height of the terrain.
    map: Vec<Vec<u8>>,
    /// The number of rows in the map.
    length: usize,
    /// The number of columns in the map.
    width: usize,
}

impl TopographicalMap {
    /// Create a new TopographicalMap instance.
    ///
    /// # Arguments
    ///   * `map` - A 2D vector representing the topographical map. The value at each cell represents the height of the terrain.
    fn new(map: Vec<Vec<u8>>) -> Self {
        let length = map.len();
        let width = map[0].len();
        Self { map, length, width }
    }

    /// Find all  possible trailheads in the map. A trailhead is a cell with a height of 0.
    /// Returns a vector of tuples representing the row and column indices of the trailheads.
    fn find_trailheads(&self) -> Vec<(usize, usize)> {
        let mut trailheads: Vec<(usize, usize)> = Vec::new();

        for (row_idx, row) in self.map.iter().enumerate() {
            for (col_idx, &height) in row.iter().enumerate() {
                if height == 0 {
                    trailheads.push((row_idx, col_idx));
                }
            }
        }

        trailheads
    }

    /// Get the valid neighbors of a cell in the map.
    /// A valid neighbor is a cell with a height that is one greater than the height of the current cell.
    /// Returns a vector of tuples representing the row and column indices of the valid neighbors.
    ///
    /// # Arguments
    ///     * `row` - The row index of the cell.
    ///     * `col` - The column index of the cell.
    fn get_valid_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut valid_neighbors: Vec<(usize, usize)> = Vec::new();
        let expected_height = self.map[row][col] + 1;

        // Check node above
        if row > 0 && self.map[row - 1][col] == expected_height {
            valid_neighbors.push((row - 1, col));
        }

        // Check node below
        if row < self.length - 1 && self.map[row + 1][col] == expected_height {
            valid_neighbors.push((row + 1, col));
        }

        // Check node to the left
        if col > 0 && self.map[row][col - 1] == expected_height {
            valid_neighbors.push((row, col - 1));
        }

        // Check node to the right
        if col < self.width - 1 && self.map[row][col + 1] == expected_height {
            valid_neighbors.push((row, col + 1));
        }

        valid_neighbors
    }

    /// Returns the trail score for a given trailhead.
    ///
    /// For a given trailhead, a trail is valid if it is a sequence of cells with increasing heights that starts at the trailhead
    /// and ends at a peak i.e. a cell with a height of 9. There are two methods to calculate the trail score:
    /// 1. Use distinct trails: The score is the number of distinct trails that start at the trailhead.
    /// 2. Do not use distinct trails: The score is the number of peaks that are reachable from the trailhead.
    ///
    /// # Arguments
    ///     * `row` - The row index of the trailhead.
    ///     * `col` - The column index of the trailhead.
    ///     * `use_distinct_trails` - A boolean flag indicating whether to use distinct trails or not.
    fn calculate_trailhead_score(&self, row: usize, col: usize, use_distinct_trails: bool) -> u64 {
        const TRAIL_PEAK: u8 = 9;
        let mut score = 0;
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        stack.push((row, col));

        while let Some((row, col)) = stack.pop() {
            if !use_distinct_trails && visited.contains(&(row, col)) {
                continue;
            }

            if self.map[row][col] == TRAIL_PEAK {
                score += 1;
            } else {
                let valid_neigbors = self.get_valid_neighbors(row, col);
                for neighbor in valid_neigbors {
                    stack.push(neighbor);
                }
            }

            if !use_distinct_trails {
                visited.insert((row, col));
            }
        }

        score
    }

    /// Returns the sum of trail scores for all trailheads in the map.
    /// See calculate_trailhead_score for details on how the score is calculated.
    ///
    /// # Arguments
    ///    * `use_distinct_trails` - A boolean flag indicating whether to use distinct trails or not.
    fn calculate_trailhead_sum(&self, use_distinct_trails: bool) -> u64 {
        let mut sum = 0;
        let trailheads = self.find_trailheads();
        for (row, col) in trailheads {
            sum += self.calculate_trailhead_score(row, col, use_distinct_trails);
        }

        sum
    }
}

fn load_input(input: String) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| match x.to_digit(10) {
                    Some(x) => x as u8,
                    None => u8::MAX,
                })
                .collect()
        })
        .collect()
}

pub fn main(file_path: String) {
    let map: Vec<Vec<u8>> = load_input(fs::read_to_string(file_path).unwrap());

    let topographical_map = TopographicalMap::new(map);
    let trailhead_sum = topographical_map.calculate_trailhead_sum(false);
    let trailhead_sum_distinct = topographical_map.calculate_trailhead_sum(true);

    println!("Trailhead Sum: {trailhead_sum}");
    println!("Trailhead Sum Distinct: {trailhead_sum_distinct}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = concat!(
            "...0...\n",
            "...1...\n",
            "...2...\n",
            "6543456\n",
            "7.....7\n",
            "8.....8\n",
            "9.....9",
        );
        let map: Vec<Vec<u8>> = load_input(String::from(input));
        let topographical_map = TopographicalMap::new(map);

        assert!(topographical_map.calculate_trailhead_sum(false) == 2);
    }

    #[test]
    fn test2() {
        let input = concat!(
            "89010123\n",
            "78121874\n",
            "87430965\n",
            "96549874\n",
            "45678903\n",
            "32019012\n",
            "01329801\n",
            "10456732",
        );
        let map: Vec<Vec<u8>> = load_input(String::from(input));
        let topographical_map = TopographicalMap::new(map);

        assert!(topographical_map.calculate_trailhead_sum(false) == 36);
        assert!(topographical_map.calculate_trailhead_sum(true) == 81);
    }

    #[test]
    fn test3() {
        let input = concat!(
            "..90..9\n",
            "...1.98\n",
            "...2..7\n",
            "6543456\n",
            "765.987\n",
            "876....\n",
            "987....",
        );
        let map: Vec<Vec<u8>> = load_input(String::from(input));
        let topographical_map = TopographicalMap::new(map);

        assert!(topographical_map.calculate_trailhead_sum(false) == 4);
        assert!(topographical_map.calculate_trailhead_sum(true) == 13);
    }

    #[test]
    fn test4() {
        let input = concat!(
            ".....0.\n",
            "..4321.\n",
            "..5..2.\n",
            "..6543.\n",
            "..7..4.\n",
            "..8765.\n",
            "..9....",
        );
        let map: Vec<Vec<u8>> = load_input(String::from(input));
        let topographical_map = TopographicalMap::new(map);

        assert!(topographical_map.calculate_trailhead_sum(true) == 3);
    }
}
