#pragma once

#include "DayX.hpp"
#include <string>

class Day1 : public DayX {
private:
  std::string m_InputFile;

protected:
  std::pair<std::string, std::string> solveImplementation() override;

public:
  explicit Day1(const std::string &input_file) : DayX(input_file) {}
};