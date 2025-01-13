use crate::common::Position;

use std::collections::{HashMap, HashSet};
use std::fs;

/// Enum to represent the four directions the guard can face.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GuardDirections {
    Up,
    Right,
    Down,
    Left,
}

/// Struct to represent the lab. The lab has the following properties:
struct Lab {
    /// Static properties of the lab.
    ///
    /// The grid of the lab. The grid is a 2D vector of characters.
    grid: Vec<Vec<char>>,
    /// The number of rows in the grid.
    row_size: usize,
    /// The number of columns in the grid.
    col_size: usize,
    /// The initial position of the guard in the grid.
    guard_initial_position: Position,
    /// The initial direction the guard is facing.
    guard_initial_direction: GuardDirections,
}

impl Lab {
    /// Create a new instance of the lab.
    ///
    /// # Arguments
    ///     grid: A 2D vector of characters representing the grid of the lab.
    fn new(grid: Vec<Vec<char>>) -> Self {
        let mut guard_position = Position { row: 0, col: 0 };
        for (row, line) in grid.iter().enumerate() {
            for (col, character) in line.iter().enumerate() {
                if *character == '^' {
                    guard_position.row = row;
                    guard_position.col = col;
                }
            }
        }

        let row_size = grid.len();
        let col_size = grid[0].len();

        Self {
            grid,
            row_size,
            col_size,
            guard_initial_position: guard_position,
            guard_initial_direction: GuardDirections::Up,
        }
    }

    /// Rotate guard by 90 degress. This is used when the guard
    /// encounters an obstacle. Returns the new direction of the guard.
    fn rotate_guard(&self, guard_direction: GuardDirections) -> GuardDirections {
        match guard_direction {
            GuardDirections::Up => GuardDirections::Right,
            GuardDirections::Right => GuardDirections::Down,
            GuardDirections::Down => GuardDirections::Left,
            GuardDirections::Left => GuardDirections::Up,
        }
    }

    /// Check if guard is leaving the map. This is based on the current
    /// direction the guard is failing. Returns true if the guard is leaving
    ///
    /// # Arguments
    ///     guard_direction: The direction the guard is facing.
    ///     guard_position: The current position of the guard.
    fn is_guard_leaving(&self, guard_direction: GuardDirections, guard_position: Position) -> bool {
        match guard_direction {
            GuardDirections::Up => guard_position.row == 0,
            GuardDirections::Right => guard_position.col == self.col_size - 1,
            GuardDirections::Down => guard_position.row == self.row_size - 1,
            GuardDirections::Left => guard_position.col == 0,
        }
    }

    // Flip the direction of the guard.
    fn flip_direction(&self, guard_direction: GuardDirections) -> GuardDirections {
        match guard_direction {
            GuardDirections::Up => GuardDirections::Down,
            GuardDirections::Right => GuardDirections::Left,
            GuardDirections::Down => GuardDirections::Up,
            GuardDirections::Left => GuardDirections::Right,
        }
    }

    /// Move guard by one unit in the current direction. Returns the new position of the guard.
    fn move_guard(&self, guard_position: Position, guard_direction: GuardDirections) -> Position {
        match guard_direction {
            GuardDirections::Up => Position::new(guard_position.row - 1, guard_position.col),
            GuardDirections::Right => Position::new(guard_position.row, guard_position.col + 1),
            GuardDirections::Down => Position::new(guard_position.row + 1, guard_position.col),
            GuardDirections::Left => Position::new(guard_position.row, guard_position.col - 1),
        }
    }

    // Try to trap the guard by placing a barrier at the new barrier position.
    // If the guard is trapped, return true. Otherwise, return false. A guard is trapped if it is
    // forced to move in a loop.
    //
    // # Arguments
    //     new_barrier: The position of the new barrier.
    fn try_trap_guard(&self, new_barrier: Position, starting_position: Position, starting_direction: GuardDirections) -> bool {
        let mut grid = self.grid.clone();
        let mut guard_position = starting_position;
        let mut guard_direction = starting_direction;
        let mut turning_positions: HashSet<(Position, GuardDirections)> = HashSet::new();

        grid[new_barrier.row][new_barrier.col] = '#';

        while self.is_guard_leaving(guard_direction, guard_position) == false {
            // Move the guard by one unit in the current direction.
            let new_position = self.move_guard(guard_position, guard_direction);

            if grid[new_position.row][new_position.col] == '#' {
                // If the guard is turning at the same position, then it is trapped.
                if turning_positions.insert((guard_position, guard_direction)) == false {
                    return true;
                }

                // Rotate the guard if it encounters a barrier.
                guard_direction = self.rotate_guard(guard_direction);
            } else {
                // update the guard position.
                guard_position = new_position;
            }
        }

        false
    }

    fn solve_guard_positions(&self) -> (usize, usize) {
        let grid = self.grid.clone();
        let mut visited_positions: HashSet<Position> = HashSet::new();
        let mut visited_directions: HashMap<Position, GuardDirections> = HashMap::new();
        let mut guard_position = self.guard_initial_position;
        let mut guard_direction = self.guard_initial_direction;
        let mut guard_traps = 0;

        // Solve path of guard.
        while self.is_guard_leaving(guard_direction, guard_position) == false {
            // Move the guard by one unit in the current direction.
            let new_position = self.move_guard(guard_position, guard_direction);

            // Track visited positions.
            if visited_positions.insert(guard_position) {
                visited_directions.insert(guard_position, guard_direction);
            }

            if grid[new_position.row][new_position.col] == '#' {

                // Rotate the guard if it encounters a barrier.
                guard_direction = self.rotate_guard(guard_direction);
            } else {
                // update the guard position.
                guard_position = new_position;
            }
        }

        // Add the last position to the visited positions.
        if visited_positions.insert(guard_position) {
            visited_directions.insert(guard_position, guard_direction);
        }

        // For each poisition visited (excluding the initial position), check if placing a barrier
        // will put the guard in a loop.
        for position in visited_positions.iter() {
            if *position == self.guard_initial_position {
                continue;
            }

            // Set starting direction to the direction the guard was facing when it visited the position,
            // the new barrier position to the position the guard visited and starting position to the position just before the barrier.
            let new_barrier_position = *position;
            let starting_direction = visited_directions[&new_barrier_position];
            let flip_direction = self.flip_direction(starting_direction);
            let starting_position = self.move_guard(new_barrier_position, flip_direction);
            if self.try_trap_guard(new_barrier_position, starting_position, starting_direction) {
                guard_traps += 1;
            }
        }

        (visited_positions.len(), guard_traps)
    }
}

pub fn main(file_path: String) {
    let grid: Vec<Vec<char>> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (guard_positions, guard_traps) = Lab::new(grid).solve_guard_positions();

    println!("Visited Positions: {guard_positions} Guard Traps: {guard_traps}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_one() {
        let input = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
        ];
        let ans = Lab::new(input).solve_guard_positions();

        assert!(ans == (41, 6));
    }

    #[test]
    fn test_case_two() {
        let input = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
        ];
        let ans = Lab::new(input).solve_guard_positions();
        assert!(ans == (7, 0));
    }

    #[test]
    fn test_case_three() {
        let input = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '^', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '#', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
        ];
        let ans = Lab::new(input).solve_guard_positions();
        assert!(ans == (14, 2));
    }
}
