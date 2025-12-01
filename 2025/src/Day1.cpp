// Day 1: Historian Hysteria
// https://adventofcode.com/2025/day/1

#include <Day1.hpp>
#include <cstdint>
#include <cstdio>
#include <string>
#include <sys/types.h>

std::pair<std::string, std::string> Day1::solveImplementation() {
  // The dial has 100 clicks on it. A full rotation is 100.
  const uint32_t TOTAL_CLICKS = 100;
  uint32_t position = 50;

  // Any time the dial passes through zero.
  uint32_t zero_crossings = 0;
  // When a rotation ends at exactly zero.
  uint32_t final_zero_positions = 0;

  // Read input using the base class utility
  auto lines = getInputLines();

  std::string part1_result = "Part 1 not implemented";
  std::string part2_result = "Part 2 not implemented";

  for (auto line : lines) {
    // Each line of format R{distance} or L{distance} e.g. R64
    //
    // distance represent the number of clicks to move on the dial to the next
    // position.
    //
    // R/L represents the direction to rotate on the dial i.e. left or right.
    auto direction = line.substr(0, 1);
    uint32_t distance = std::stoi(line.substr(1, line.length()));

    // The distance may exceed the number of available ticks on the dial before
    // hitting the current position. revolutions represents the number of times
    // the current position is reached again during the current rotation.
    //
    uint32_t revolutions = distance / TOTAL_CLICKS;
    // The distance left without any more full rotations.
    uint32_t clicks = distance % TOTAL_CLICKS;

    // Each full revolution back to the current position will be a zero
    // crossing.
    zero_crossings += revolutions;

    //
    // Even though clicks has been bounded to be within TOTAL_CLICKS. A
    // rotation may still involve a zero crossing depending on the current
    // position.
    //

    if (direction == "L") {
      if (clicks > position && position != 0) {
        // For left rotation. If clicks > position, and position is not current
        // 0 there will be a zero crossing.
        zero_crossings += 1;
      }
      // A left rotation of N clicks would be the same as a right rotation
      // of (TOTAL_CLICKS - N) clicks. Using this for convenience of avoiding
      // substractions.
      clicks = TOTAL_CLICKS - clicks;
    } else if (direction == "R") {
      if (clicks > (TOTAL_CLICKS - position)) {
        // For right rotation. If clicks > TOTAL_CLICKS - position, there
        // will be a zero crossing.
        zero_crossings += 1;
      }
    }

    uint32_t new_position = position + clicks;
    // actual position will be (new_position % TOTAL_CLICKS)
    position = new_position % TOTAL_CLICKS;

    if (position == 0) {
      final_zero_positions += 1;
      zero_crossings += 1;
    }
  }

  if (!lines.empty()) {
    part1_result = std::to_string(final_zero_positions);
    part2_result = std::to_string(zero_crossings);
  }

  return std::make_pair(part1_result, part2_result);
}
