// Day 8: Playground
// https://adventofcode.com/2025/day/8

#include <DayX.hpp>
#include <cassert>
#include <cstddef>
#include <cstdint>
#include <list>
#include <map>
#include <set>
#include <utility>
#include <vector>

std::pair<uint64_t, uint64_t> Day8::solveImplementation() {
  // Read input using the base class utility
  std::vector<std::string> lines = getInputLines();

  // Store all positions.
  std::vector<Position3D> positions;
  // Store all the distances.
  std::map<double, std::pair<Position3D, Position3D>> distances;
  // Store all circuits as sets of positions
  std::list<std::set<Position3D>> circuits;
  // Store map of position to circuit it is in
  std::map<Position3D, std::list<std::set<Position3D>>::iterator>
      positionCircuitMap;
  // Store 3 larges circuits
  std::map<size_t, std::list<std::set<Position3D>>::iterator> largestCircuits;

  uint64_t part_1 = 1;
  uint64_t part_2 = 0;

  // Convert string inputs of positions to string type.
  for (auto &position_str : lines) {
    std::vector<int64_t> position_vec = splitString<int64_t>(position_str, ',');
    assert(position_vec.size() == 3);

    Position3D position{position_vec[0], position_vec[1], position_vec[2]};
    positions.push_back(position);
  }

  // Precompute all the distances into ordered map.
  for (size_t idx = 0; idx < positions.size(); idx++) {
    for (size_t jdx = 0; jdx < positions.size(); jdx++) {
      if (idx == jdx) {
        continue;
      }
      double distance = positions[idx].distance(positions[jdx]);
      distances[distance] = {positions[idx], positions[jdx]};
    }
  }

  size_t checked = 0;
  for (auto &[distance, postions] : distances) {
    auto [p1, p2] = postions;
    std::list<std::set<Position3D>>::iterator it;

    if (positionCircuitMap.contains(p1) && !positionCircuitMap.contains(p2)) {
      // If p1 in a circuit but p2 is not, add p2 to p1's circuit
      it = positionCircuitMap[p1];
      it->insert(p2);
      positionCircuitMap[p2] = it;
    } else if (positionCircuitMap.contains(p2) &&
               !positionCircuitMap.contains(p1)) {
      // If p2 in a circuit but p1 is not, add p1 to p2's circuit
      it = positionCircuitMap[p2];
      it->insert(p1);
      positionCircuitMap[p1] = it;
    } else if (positionCircuitMap.contains(p1) &&
               positionCircuitMap.contains(p2)) {
      // If both p1 and p2 are in a circuit but not in the same one, combine
      // the circuit
      auto it1 = positionCircuitMap[p1];
      auto it2 = positionCircuitMap[p2];

      if (it1 != it2) {
        for (const auto &position : *it2) {
          positionCircuitMap[position] = it1;
        }
        it1->insert(it2->begin(), it2->end());
        circuits.erase(it2);
      }

      it = it1;
    } else {
      // If p1 and p2 are not yet in a circuilt, create new one.
      std::set<Position3D> new_circuit = {p1, p2};

      circuits.push_back(new_circuit);
      it = std::prev(circuits.end());
      positionCircuitMap[p1] = it;
      positionCircuitMap[p2] = it;
    }

    if (it->size() == positions.size()) {
      // Compute part 2.
      // At this point all positions are connected in a single circuit
      part_2 = p1.x * p2.x;
      break;
    }

    if (checked == 1000) {
      // Compute part 1.
      // After 1000 connections
      circuits.sort(
          [](const std::set<Position3D> &c1, const std::set<Position3D> &c2) {
            return c1.size() > c2.size();
          });

      size_t count = 0;
      for (auto &circuit : circuits) {
        if (count >= 3) {
          // We only care about multiply the size of the 3 largest circuits.
          break;
        }
        part_1 *= circuit.size();
        count++;
      }
    }

    checked++;
  }

  return {part_1, part_2};
}