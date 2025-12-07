#pragma once

#include <cstddef>
#include <cstdint>
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
  using position = std::pair<size_t, size_t>;

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