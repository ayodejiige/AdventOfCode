#include <DayX.hpp>
#include <fstream>
#include <stdexcept>
#include <string>

std::vector<std::string> DayX::getInputLines() {
  std::vector<std::string> lines;
  std::ifstream file(m_InputFile);

  if (!file.is_open()) {
    throw std::runtime_error("Could not open file: " + m_InputFile);
  }

  std::string line;
  while (std::getline(file, line)) {
    lines.push_back(line);
  }

  return lines;
}