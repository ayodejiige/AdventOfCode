// Day 6: Trash Compactor
// https://adventofcode.com/2025/day/6

#include <DayX.hpp>
#include <cstdint>
#include <stdexcept>
#include <string>
#include <vector>
#include <xlocale/_stdio.h>

std::pair<uint64_t, uint64_t> Day6::solveImplementation() {
  uint64_t part_1 = 0;
  uint64_t part_2 = 0;

  // Read input using the base class utility
  std::vector<std::string> lines = getInputLines();
  // Row size excluding the operators.
  size_t row_size = lines.size() - 1;
  // List of operators
  std::vector<char> operators = splitString<char>(lines[row_size], ' ');
  // Vector to accumulate the solution for each operation.
  std::vector<uint64_t> accumulator(operators.size());

  // Part 1

  // Iterate through each row of numbers.
  for (size_t row = 0; row < row_size; row++) {
    std::vector<uint64_t> numbers = splitString<uint64_t>(lines[row], ' ');
    // Go through each col in the row and accumulate result in the accumulator.
    for (size_t col = 0; col < operators.size(); col++) {
      if (row == 0) {
        // First set of numbers do not require any operations.
        accumulator[col] = numbers[col];
      } else {
        if (operators[col] == '+') {
          accumulator[col] += numbers[col];
        } else if (operators[col] == '*') {
          accumulator[col] *= numbers[col];
        } else {
          throw std::runtime_error("Invalid operator -> " +
                                   std::to_string(operators[col]));
        }
      }

      if (row == row_size - 1) {
        // If this is the last row of numbers. Add to sum total.
        part_1 += accumulator[col];
      }
    }
  }

  // Part 2
  uint64_t current_operation_accumulator = 0;
  size_t current_operator_index = operators.size() - 1;
  bool is_first_in_current_operation = true;

  // Iterate through each column of characters from the right.
  for (int char_col = static_cast<int>(lines[0].size() - 1); char_col >= 0;
       char_col--) {
    // Stores the number formed by the digits in the current column.
    uint64_t current_num = 0;
    // Tracks if the column has no digits.
    bool is_empty_column = true;

    // Iterate through each row from the top to build the number
    for (size_t row = 0; row < row_size; row++) {
      auto c = lines[row][char_col];
      if (c != ' ') {
        current_num = current_num * 10 + (c - '0');
        is_empty_column = false;
      }
    }

    if (is_empty_column == false) {
      // If the current column has digits, carry out the operation.
      if (is_first_in_current_operation) {
        // First set of numbers do not require any operations.
        current_operation_accumulator = current_num;
        is_first_in_current_operation = false;
      } else {
        if (operators[current_operator_index] == '+') {
          current_operation_accumulator += current_num;
        } else if (operators[current_operator_index] == '*') {
          current_operation_accumulator *= current_num;
        } else {
          throw std::runtime_error(
              "Invalid operator -> " +
              std::to_string(operators[current_operator_index]));
        }
      }
    } else {
      // An empty columns means we can move to the next set of operations.
      part_2 += current_operation_accumulator;
      current_operator_index--;
      is_first_in_current_operation = true;
    }
  }

  // Add the last accumulated value for part 2
  part_2 += current_operation_accumulator;

  return {part_1, part_2};
}