// Day 5: Cafeteria
// https://adventofcode.com/2025/day/5

#include <DayX.hpp>
#include <algorithm>
#include <cstdint>
#include <string>
#include <vector>

std::pair<uint64_t, uint64_t> Day5::solveImplementation() {
  std::vector<std::string> lines = getInputLines();

  // Count of IDs that fall within any range
  uint64_t in_range_count = 0;
  // Total number of values covered by all merged ranges
  uint64_t range_size = 0;
  // Original ranges from input
  std::vector<range> ranges;
  // Non-overlapping ranges after merging
  std::vector<range> merged_ranges;

  // Parse ranges from input until we hit an empty line
  // Input format: each line contains "start-end" (e.g., "10-20")
  auto it_input = lines.begin();
  while (*it_input != "") {
    std::vector<uint64_t> parts = splitString<uint64_t>(*it_input++, '-');
    range r = {parts[0], parts[1]};
    ranges.push_back(r);
  }

  // Sort ranges by start position to enable efficient merging
  std::sort(ranges.begin(), ranges.end());

  // Merge overlapping or adjacent ranges to eliminate redundancy
  auto it_range = ranges.begin();
  range current_range = *it_range;
  while (++it_range != ranges.end()) {
    // Check if current range overlaps with or is adjacent to the next range
    // Two ranges overlap/touch if: current.end >= next.start
    if (it_range->first <= current_range.second) {
      // Ranges overlap, extend current range to include the overlapping range
      current_range.second = std::max(it_range->second, current_range.second);
    } else {
      // No overlap,save current range and accumulate its size
      range_size += current_range.second - current_range.first + 1;
      merged_ranges.push_back(current_range);
      current_range = *it_range;
    }
  }

  // Add the final range after the loop ends
  range_size += current_range.second - current_range.first + 1;
  merged_ranges.push_back(current_range);

  // Process the list of IDs to check which ones fall within our merged ranges
  // Each ID is on a separate line after the empty line separator
  while (++it_input != lines.end()) {
    uint64_t id = std::stoll(*it_input);

    // Use binary search to efficiently find the appropriate range
    // lower_bound finds the first range where start > id
    auto it_range = std::upper_bound(
        merged_ranges.begin(), merged_ranges.end(), std::make_pair(id, id),
        [](const range &element, const range &value) {
          return element.first < value.first;
        });

    // Check if ID falls within any range by looking at the previous range
    // (since lower_bound returns first element >= target)
    if (it_range != merged_ranges.begin()) {
      --it_range;
      if (id >= it_range->first && id <= it_range->second) {
        in_range_count++;
      }
    }
  }

  return {in_range_count, range_size};
}