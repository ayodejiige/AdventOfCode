#pragma once

#include <string>

class DayX {
public:
  virtual ~DayX() = default;
  virtual std::string part1() = 0;
  virtual std::string part2() = 0;
};