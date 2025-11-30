#pragma once

#include "DayX.hpp"
#include <string>

class Day1 : public DayX {
private:
  std::string m_InputFile;

public:
  explicit Day1(const std::string &input_file) : m_InputFile(input_file) {};
  std::string part1() override;
  std::string part2() override;
};