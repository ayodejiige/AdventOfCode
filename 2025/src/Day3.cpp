// Day 3: Lobby
// https://adventofcode.com/2025/day/3

#include <DayX.hpp>
#include <cstdint>
#include <cstdio>
#include <string>
#include <vector>

// Given a bank (i.e. string representing a list of digits, each digit
// corresponding to the joltage of a single battery). Find the largest jolt
// possible. Where a jolt is the combination of N battery jolts in order.
//
// e.g. N: 2, Bank: 1 2 3 8 0 9 0 1, Largest Jolt: 91
uint64_t Day3::largestJolt(std::string bank, uint32_t battery_count) {
  if (bank.length() < battery_count) {
    return std::stoi(bank);
  }

  // Populate the initial jolts with the first N batteries.
  std::vector<uint32_t> joltsPosition;
  for (uint32_t idx = 0; idx < battery_count; idx++) {
    joltsPosition.push_back(idx);
  }

  // Iterate through the bank starting from the (N+1)th battery.
  uint32_t current_battery_idx = 1;
  while (current_battery_idx < bank.length()) {
    uint32_t current_digit = bank[current_battery_idx] - '0';

    // Check if the current digit can replace any of the existing jolts.
    for (uint32_t idx = 0; idx < current_battery_idx; idx++) {
      if (bank.length() - current_battery_idx < joltsPosition.size() - idx) {
        // Skip this index when there are not enough batteries left to update
        // the idx and the subsequent ones.
        continue;
      }

      if (joltsPosition[idx] > current_battery_idx) {
        // idx is out of range of current batteries being considered.
        break;
      }

      if (current_digit > bank[joltsPosition[idx]] - '0') {
        // Replace the jolt at idx with the current digit.
        joltsPosition[idx] = current_battery_idx;

        // Update all subsequent jolts with the next batteries in the bank.
        for (uint32_t j = idx + 1; j < joltsPosition.size(); j++) {
          joltsPosition[j] = current_battery_idx + (j - idx);
        }

        break;
      }
    }
    current_battery_idx++;
  }

  // Calculate the largest jolt from the selected batteries.
  uint64_t max_jolts = 0;
  uint32_t idx = 0;
  for (auto it = joltsPosition.rbegin(); it != joltsPosition.rend(); ++it) {
    max_jolts += (bank[*it] - '0') * std::pow(10, idx);
    idx++;
  }

  return max_jolts;
}

std::pair<std::string, std::string> Day3::solveImplementation() {
  uint64_t part_1 = 0;
  uint64_t part_2 = 0;

  // Read input using the base class utility
  std::vector<std::string> banks = getInputLines();

  for (auto const &bank : banks) {
    printf("Testing %s\n", bank.c_str());
    part_1 += largestJolt(bank, 2);
    part_2 += largestJolt(bank, 12);
  }

  std::string part1_result = std::to_string(part_1);
  std::string part2_result = std::to_string(part_2);

  return std::make_pair(part1_result, part2_result);
}