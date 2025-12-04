#pragma once

#include <cstdint>
#include <set>
#include <string>
#include <utility>
#include <vector>

class DayX {
private:
  std::string m_Part1 = "";
  std::string m_Part2 = "";
  std::string m_InputFile;

protected:
  virtual std::pair<std::string, std::string> solveImplementation() = 0;

  std::vector<std::string> getInputLines();
  std::vector<std::string> splitString(const std::string &str, char delimiter);

public:
  explicit DayX(const std::string &input_file) : m_InputFile(input_file) {}
  virtual ~DayX() = default;

  void solve() {
    auto [part1, part2] = solveImplementation();
    m_Part1 = part1;
    m_Part2 = part2;
  }

  std::string part1() { return m_Part1; };
  std::string part2() { return m_Part2; };
};

class Day1 : public DayX {
private:
  using DayX::DayX;

protected:
  std::pair<std::string, std::string> solveImplementation() override;
};

class Day2 : public DayX {
private:
  using DayX::DayX;
  bool isRepeatedTwice(uint64_t id);
  std::pair<uint64_t, uint64_t> repeatedSums(uint64_t start, uint64_t end);

protected:
  std::pair<std::string, std::string> solveImplementation() override;
};

class Day3 : public DayX {
private:
  using DayX::DayX;
  uint64_t largestJolt(const std::string &bank, uint32_t battery_count);

protected:
  std::pair<std::string, std::string> solveImplementation() override;
};

class Day4 : public DayX {
private:
  using position = std::pair<int32_t, int32_t>;
  using DayX::DayX;
  bool canBeRemoved(const std::vector<std::string> &grid, int32_t row,
                    int32_t col, std::set<position> &neighbors);

protected:
  std::pair<std::string, std::string> solveImplementation() override;
};