#include <DayX.hpp>
#include <fstream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

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

std::vector<std::string> DayX::splitString(const std::string &input,
                                           char delimiter) {
  std::vector<std::string> tokens;
  std::istringstream input_stream(input);
  std::string token;

  while (std::getline(input_stream, token, delimiter)) {
    tokens.push_back(token);
  }

  return tokens;
}