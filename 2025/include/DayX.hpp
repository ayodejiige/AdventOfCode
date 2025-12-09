#pragma once

#include <cstddef>
#include <cstdint>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <set>
#include <string>
#include <utility>
#include <vector>

class DayX {
private:
  uint64_t m_Part1;
  uint64_t m_Part2;
  std::string m_InputFile;

protected:
  using position = std::pair<int64_t, int64_t>;
  virtual std::pair<uint64_t, uint64_t> solveImplementation() = 0;

  std::vector<std::string> getInputLines();
  template <typename T>
  std::vector<T> splitString(const std::string &str, char delimiter);

public:
  explicit DayX(const std::string &input_file) : m_InputFile(input_file) {}
  virtual ~DayX() = default;

  void solve() {
    auto [part1, part2] = solveImplementation();
    m_Part1 = part1;
    m_Part2 = part2;
  }

  uint64_t part1() { return m_Part1; };
  uint64_t part2() { return m_Part2; };
};

class Day1 : public DayX {
private:
  using DayX::DayX;

protected:
  std::pair<uint64_t, uint64_t> solveImplementation() override;
};

class Day2 : public DayX {
private:
  using DayX::DayX;
  bool isRepeatedTwice(uint64_t id);
  std::pair<uint64_t, uint64_t> repeatedSums(uint64_t start, uint64_t end);

protected:
  std::pair<uint64_t, uint64_t> solveImplementation() override;
};

class Day3 : public DayX {
private:
  using DayX::DayX;
  uint64_t largestJolt(const std::string &bank, uint32_t battery_count);

protected:
  std::pair<uint64_t, uint64_t> solveImplementation() override;
};

class Day4 : public DayX {
private:
  using DayX::DayX;
  bool canBeRemoved(const std::vector<std::string> &grid, size_t row,
                    size_t col, std::set<position> &neighbors);

protected:
  std::pair<uint64_t, uint64_t> solveImplementation() override;
};

class Day5 : public DayX {
  using range = std::pair<uint64_t, uint64_t>;

private:
  using DayX::DayX;

protected:
  std::pair<uint64_t, uint64_t> solveImplementation() override;
};

class Day6 : public DayX {
private:
  using DayX::DayX;

protected:
  std::pair<uint64_t, uint64_t> solveImplementation() override;
};

class Day7 : public DayX {
private:
  using DayX::DayX;

  void continueBeam(position old_position, position new_postion,
                    std::map<position, uint64_t> &visited,
                    std::queue<position> &beams);

public:
  std::pair<uint64_t, uint64_t> solveImplementation() override;
};

class Day8 : public DayX {
private:
  using DayX::DayX;

public:
  std::pair<uint64_t, uint64_t> solveImplementation() override;
};

// Utilities

// Pair printing
template <typename T1, typename T2>
std::ostream &operator<<(std::ostream &os, const std::pair<T1, T2> &p) {
  return os << "(" << p.first << "," << p.second << ")";
}

template <typename T> struct is_container : std::false_type {};
template <typename T> struct is_container<std::vector<T>> : std::true_type {};
template <typename T> struct is_container<std::set<T>> : std::true_type {};
template <typename T> struct is_container<std::list<T>> : std::true_type {};

// Container printing
template <typename Container>
typename std::enable_if<is_container<Container>::value, std::ostream &>::type
operator<<(std::ostream &os, const Container &container) {
  os << "[";
  bool first = true;
  for (const auto &item : container) {
    if (!first)
      os << ", ";
    os << item;
    first = false;
  }
  return os << "]";
}

// Map printing
template <typename K, typename V>
std::ostream &operator<<(std::ostream &os, const std::map<K, V> &map) {
  os << "{";
  bool first = true;
  for (const auto &[key, value] : map) {
    if (!first)
      os << ", ";
    os << key << ": " << value;
    first = false;
  }
  return os << "}";
}

struct Position3D {
  int64_t x, y, z;
  bool operator<(const Position3D &other) const;
  double distance(const Position3D &other) const;
};

std::ostream &operator<<(std::ostream &os, const Position3D &position);