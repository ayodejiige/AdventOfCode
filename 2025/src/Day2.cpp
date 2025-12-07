// Day 2: Gift Shop
// https://adventofcode.com/2025/day/2

#include <DayX.hpp>
#include <cmath>
#include <cstdint>
#include <set>
#include <string>
#include <vector>

// Returns true if the id consists of a sequence of digits repeated twice
// e.g. 1212, 7878, 123123
bool Day2::isRepeatedTwice(uint64_t id) {
  uint64_t length = std::floor(std::log10(id)) + 1;
  if (length % 2 != 0) {
    return false;
  }

  uint64_t half_length = length / 2;
  uint64_t divisor = static_cast<uint64_t>(std::pow(10, half_length));

  return id / divisor == id % divisor;
}

// Returns a pair of sums of specific numbers between start and end (inclusive):
//  - first: sum of all numbers that are formed by a sequence of digits repeated
//  twice
//  - second: sum of all numbers that are formed by any sequence of digits
//  repeated
std::pair<uint64_t, uint64_t> Day2::repeatedSums(uint64_t start, uint64_t end) {
  // Store repeated sequences to avoid double counting.
  std::set<uint64_t> visited_sequences;
  uint64_t any_repeated_sum = 0;
  uint64_t twice_repeated_sum = 0;

  // Get the length of each number.
  uint64_t start_length = std::floor(std::log10(start)) + 1;
  uint64_t end_length = std::floor(std::log10(end)) + 1;

  // Iterate through all possible full lengths.
  for (uint64_t total_length = start_length; total_length <= end_length;
       total_length++) {

    //  Iterate through all possible sequence lengths that can form the
    //  full length when repeated.
    for (uint64_t sequence_length = 1; sequence_length <= total_length / 2;
         sequence_length++) {

      if (total_length % sequence_length != 0) {
        // Sequence length must evenly divide the total length.
        continue;
      }

      uint64_t min_sequence_val = std::pow(10, sequence_length - 1);
      uint64_t max_sequence_val = std::pow(10, sequence_length) - 1;

      // Iterate through all possible sequence values of the given length.
      for (uint64_t sequence_val = min_sequence_val;
           sequence_val <= max_sequence_val; sequence_val++) {
        uint64_t repeated_sequence = 0;

        // Form the repeated sequence.
        for (uint64_t i = 0; i < total_length / sequence_length; i++) {
          repeated_sequence += sequence_val * std::pow(10, i * sequence_length);
        }

        if (repeated_sequence >= start && repeated_sequence <= end &&
            visited_sequences.find(repeated_sequence) ==
                visited_sequences.end()) {
          visited_sequences.insert(repeated_sequence);
          any_repeated_sum += repeated_sequence;

          if (isRepeatedTwice(repeated_sequence)) {
            // For part 1, only sequences that are repeated twice are invalid.
            twice_repeated_sum += repeated_sequence;
          }
        }
      }
    }
  }

  return std::make_pair(twice_repeated_sum, any_repeated_sum);
}

std::pair<uint64_t, uint64_t> Day2::solveImplementation() {
  uint64_t twice_repeated_sums = 0;
  uint64_t any_repeated_sums = 0;

  // Read input using the base class utility
  std::vector<std::string> lines = getInputLines();

  std::vector<std::string> ranges = splitString<std::string>(lines[0], ',');

  for (const auto &range : ranges) {
    std::vector<uint64_t> range_pair = splitString<uint64_t>(range, '-');
    uint64_t start = range_pair[0];
    uint64_t end = range_pair[1];

    auto [double_sequence_sum, any_repeated_sum] = repeatedSums(start, end);

    twice_repeated_sums += double_sequence_sum;
    any_repeated_sums += any_repeated_sum;
  }

  return {twice_repeated_sums, any_repeated_sums};
}
