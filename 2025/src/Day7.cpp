// Day 7: Laboratories
// https://adventofcode.com/2025/day/7

#include <DayX.hpp>
#include <cstdint>
#include <map>
#include <queue>
#include <stdexcept>
#include <string>

void Day7::continueBeam(position old_position, position new_position,
                        std::map<position, uint64_t> &visited,
                        std::queue<position> &beams) {
  if (visited.contains(new_position)) {
    // Beam has reached this position before, accumulate paths.
    visited[new_position] += visited[old_position];
  } else {
    // First time beam reaches this position.
    visited[new_position] = visited[old_position];
    beams.push(new_position);
  }
}

std::pair<uint64_t, uint64_t> Day7::solveImplementation() {
  const char ENTRY = 'S';
  const char SPLITTER = '^';
  const char SPACE = '.';

  uint64_t split_count = 0;
  uint64_t timeline_count = 0;

  // Read input using the base class utility
  std::vector<std::string> grid = getInputLines();
  const size_t ROW_SIZE = grid.size();
  const size_t COL_SIZE = grid[0].size();

  // Tracks positions in the grid where beams currently are.
  std::queue<position> beams;
  // Track all visited positions and the number of paths leading to it.
  std::map<position, uint64_t> visited;

  // Find the S in the grid
  for (int64_t row = 0; row < ROW_SIZE; row++) {
    for (int64_t col = 0; col < COL_SIZE; col++) {
      if (grid[row][col] == 'S') {
        beams.push({row, col});
        visited[{row, col}] = 1;
        break;
      }
    }
  }

  if (beams.size() == 0) {
    throw std::runtime_error("No entry found in grid");
  }

  // Pass beams through grid until they exit or are completely blocked
  while (beams.size() != 0) {
    position new_position;
    position old_position = beams.front();
    beams.pop();

    char current_cell = grid[old_position.first][old_position.second];

    if (current_cell == SPACE || current_cell == ENTRY) {
      // Move straight down if current cell is empty space or entry point.
      int64_t new_row = old_position.first + 1;

      if (new_row < ROW_SIZE) {
        // New position is still within grid bounds, continue beam.
        new_position = {new_row, old_position.second};
        continueBeam(old_position, new_position, visited, beams);
      } else {
        // Beam exits grid at the bottom.
        timeline_count += visited[old_position];
      }
    } else if (current_cell == SPLITTER) {
      // Split beam into two directions if current cell is a splitter.
      bool split_happened = false;
      int64_t new_row = old_position.first + 1;
      int64_t left_col = old_position.second - 1;
      int64_t right_col = old_position.second + 1;

      if (left_col >= 0 && new_row < ROW_SIZE) {
        // Left position is within grid bounds, continue beam to the left.
        new_position = {new_row, left_col};
        continueBeam(old_position, new_position, visited, beams);

        split_happened = true;
      }

      if (right_col < COL_SIZE && new_row < ROW_SIZE) {
        // Right position is within grid bounds, continue beam to the right.
        new_position = {new_row, right_col};
        continueBeam(old_position, new_position, visited, beams);
        split_happened = true;
      }

      if (split_happened == false) {
        // Both split directions are out of bounds, beam exits grid.
        timeline_count += visited[old_position];
      }

      split_count++;

    } else {
      throw std::runtime_error("Unexpected character in grid " +
                               std::to_string(current_cell));
    }
  }

  return {split_count, timeline_count};
}
