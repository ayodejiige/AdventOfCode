#pragma once

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