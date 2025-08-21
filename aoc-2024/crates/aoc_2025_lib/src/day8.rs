use crate::common::Position;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

/// Struct to represent the city. Specifically antennas on the roofs of buildings.
struct City {
    /// 2D vector representing the city with antennas on the roofs of buildings.
    map: Vec<Vec<char>>,
    // The number of rows in the city map.
    length: usize,
    // The number of cols in the city map.
    width: usize,
}

impl City {
    /// Create a new city with the given antenna map.
    fn new(map: Vec<Vec<char>>) -> City {
        let length = map.len();
        let width = map[0].len();
        City { map, length, width }
    }

    /// Give a row and col, check if it is within the boundaries of the city.
    fn is_in_city(&self, row: isize, col: isize) -> bool {
        return row >= 0 && row < self.length as isize && col >= 0 && col < self.width as isize;
    }

    /// Given a pair of satellites of the same kind. Find distinct antinodes.
    fn count_antenna_pair_antinodes(
        &self,
        antenna_a: &Position,
        antenna_b: &Position,
        antinodes: &mut HashSet<Position>,
    ) {
        let a_row = antenna_a.row as isize;
        let a_col = antenna_a.col as isize;
        let b_row = antenna_b.row as isize;
        let b_col = antenna_b.col as isize;

        // Calculate distance between two antennas.
        let row_distance = a_row - b_row;
        let col_distance = a_col - b_col;

        // Calculate location of antinodes
        let mut antinode_one_row = a_row + row_distance;
        let mut antinode_two_row = b_row - row_distance;
        let mut antinode_one_col = a_col + col_distance;
        let mut antinode_two_col = b_col - col_distance;

        while self.is_in_city(antinode_one_row, antinode_one_col) {
            antinodes.insert(Position::new(
                antinode_one_row as usize,
                antinode_one_col as usize,
            ));
            antinode_one_row += row_distance;
            antinode_one_col += col_distance;
        }

        while self.is_in_city(antinode_two_row, antinode_two_col) {
            antinodes.insert(Position::new(
                antinode_two_row as usize,
                antinode_two_col as usize,
            ));
            antinode_two_row += row_distance;
            antinode_two_col += col_distance;
        }
    }

    /// Count the number of antinodes in the city. An antinode occurs at any point that is perfectly in
    /// line with two antennas of the same kind.
    fn count_antinodes(&self) -> usize {
        let mut antenna_map: HashMap<char, Vec<Position>> = HashMap::new();
        let mut antinodes: HashSet<Position> = HashSet::new(); // to store unique antinodes.

        // Group antennas of the same kind into a map.
        for (row, line) in self
            .map
            .iter()
            .enumerate()
        {
            for (col, node) in line
                .iter()
                .enumerate()
            {
                if *node == '.' {
                    // Node is not an antenna
                    continue;
                }

                if !antenna_map.contains_key(node) {
                    antenna_map.insert(*node, Vec::new());
                }

                antenna_map
                    .get_mut(node)
                    .unwrap()
                    .push(Position::new(row, col));
            }
        }

        // For each kind of antenna, count the possible antinodes for each pair of distinct antennas.
        for key in antenna_map.keys() {
            let matching_antennas = antenna_map
                .get(key)
                .unwrap();

            // Any two antennas of the same kind will form a line. Loop through each pair of
            // distinct antennas of the same kind to find the antinodes within the boundaries of the city
            for antenna_a in matching_antennas {
                for antenna_b in matching_antennas {
                    if antenna_a == antenna_b {
                        // One antenna cannot form a line.
                        continue;
                    }
                    // The two antennas in a pair are also antinodes since they are inline with one another.
                    antinodes.insert(*antenna_a);
                    antinodes.insert(*antenna_b);

                    self.count_antenna_pair_antinodes(antenna_a, antenna_b, &mut antinodes);
                }
            }
        }

        antinodes.len()
    }
}

pub fn main(file_path: String) {
    // Read file path and parse into 2d array fo vectors
    let map: Vec<Vec<char>> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .collect()
        })
        .collect();

    let antinodes_count = City::new(map).count_antinodes();

    println!("Antinodes: {antinodes_count}");
}
