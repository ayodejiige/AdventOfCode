// Day 4: Printing Department
// https://adventofcode.com/2025/day/4

#include <DayX.hpp>
#include <cstdint>
#include <map>
#include <set>
#include <string>
#include <utility>
#include <vector>

// Returns true if the roll in the given row and col in the grid can be
// removed. A roll can be removed if there are less than 4 neighboring rolls
// around it.
bool Day4::canBeRemoved(const std::vector<std::string> &grid, int32_t row,
                        int32_t col, std::set<position> &neighbors) {
  static const std::vector<position> DIRECTIONS = {
      {-1, -1}, // Top-left
      {-1, 0},  // Top
      {-1, 1},  // Top-right
      {0, -1},  // Left
      {0, 1},   // Right
      {1, -1},  // Bottom-left
      {1, 0},   // Bottom
      {1, 1}    // Bottom-right
  };

  int32_t row_max = grid.size();
  int32_t col_max = grid[0].size();

  uint32_t roll_count = 0;

  // For the current position, go through each adjacent position to check if
  // there are no rolls.
  for (const auto &[row_dir, col_dir] : DIRECTIONS) {
    int32_t new_row = row + row_dir;
    int32_t new_col = col + col_dir;

    if (new_row < 0 || new_row >= row_max || new_col < 0 ||
        new_col >= col_max) {
      // Adjacent position is out of bounds.
      continue;
    }

    if (grid[new_row][new_col] == '@') {
      neighbors.insert({new_row, new_col});
      roll_count++;
    }
  }

  return roll_count < 4;
}

std::pair<std::string, std::string> Day4::solveImplementation() {
  uint32_t part1_count = 0;
  uint32_t part2_count = 0;
  std::map<position, std::set<position>> removed_rolls;
  std::map<position, std::set<position>> remaining_rolls;

  // Read input using the base class utility
  std::vector<std::string> grid = getInputLines();

  for (size_t row = 0; row < grid.size(); row++) {
    for (size_t col = 0; col < grid[0].length(); col++) {

      if (grid[row][col] != '@') {
        // If current position is not a roll, then we can skip it.
        continue;
      }

      // Tracks the rolls around the current position
      std::set<std::pair<int32_t, int32_t>> neighbors;

      if (canBeRemoved(grid, row, col, neighbors)) {
        removed_rolls[{row, col}] = neighbors;
        part1_count++;
      } else {
        remaining_rolls[{row, col}] = neighbors;
      }
    }
  }

  // Add first set of removed rolls to part 2.
  part2_count = part1_count;

  // Iterate through removing rolls until there are no rolls left to remove.
  while (!removed_rolls.empty()) {
    // Iterate through each removed roll and update the neighbors.
    for (auto it = removed_rolls.begin(); it != removed_rolls.end();) {
      const auto &[position, neighbors] = *it;

      // Update each neighbor of a removed roll to no longer include the removed
      // roll
      for (auto &neighbor : neighbors) {
        if (remaining_rolls.find(neighbor) == remaining_rolls.end()) {
          continue;
        }
        remaining_rolls[neighbor].erase(position);
      }

      it = removed_rolls.erase(it);
    }

    // Iterate through the remaining rolls to see if there are more rolls to be
    // removed.
    for (auto it = remaining_rolls.begin(); it != remaining_rolls.end();) {
      const auto &[position, neighbors] = *it;
      if (neighbors.size() < 4) {
        removed_rolls[position] = neighbors;
        it = remaining_rolls.erase(it);
        part2_count++;
      } else {
        it++;
      }
    }
  }

  std::string part1_result = std::to_string(part1_count);
  std::string part2_result = std::to_string(part2_count);

  return {part1_result, part2_result};
}