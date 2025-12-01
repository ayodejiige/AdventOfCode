#pragma once

#include "DayX.hpp"
#include <string>

class Day1 : public DayX {
private:

protected:
  std::pair<std::string, std::string> solveImplementation() override;

public:
  explicit Day1(const std::string &input_file) : DayX(input_file) {}
};