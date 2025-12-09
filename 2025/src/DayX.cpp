#include <DayX.hpp>
#include <cctype>
#include <fstream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <type_traits>
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

template <typename T>
std::vector<T> DayX::splitString(const std::string &input, char delimiter) {
  std::vector<T> tokens;
  std::istringstream input_stream(input);
  std::string token;

  while (std::getline(input_stream, token, delimiter)) {
    // Trim white space
    token.erase(std::remove_if(token.begin(), token.end(),
                               [](char c) { return std::isspace(c); }),
                token.end());

    // Skip empty value after whitespace trimming.
    if (token == "") {
      continue;
    }

    if constexpr (std::is_same_v<T, std::string>) {
      tokens.push_back(token);
    } else if constexpr (std::is_same_v<T, char>) {
      tokens.push_back(token[0]);
    } else if constexpr (std::is_integral_v<T>) {
      if constexpr (std::is_signed_v<T>) {
        tokens.push_back(static_cast<T>(std::stoll(token)));
      } else {
        tokens.push_back(static_cast<T>(std::stoull(token)));
      }
    } else {
      static_assert(false, "Unsupported type for splitString");
    }
  }
  return tokens;
}

template std::vector<std::string>
DayX::splitString<std::string>(const std::string &, char);
template std::vector<char> DayX::splitString<char>(const std::string &, char);
template std::vector<uint64_t> DayX::splitString<uint64_t>(const std::string &,
                                                           char);
template std::vector<int64_t> DayX::splitString<int64_t>(const std::string &,
                                                         char);

double Position3D::distance(const Position3D &other) const {
  int64_t x_diff = x - other.x;
  int64_t y_diff = y - other.y;
  int64_t z_diff = z - other.z;

  return std::sqrt(x_diff * x_diff + y_diff * y_diff + z_diff * z_diff);
}

bool Position3D::operator<(const Position3D &other) const {
  Position3D zero = {0, 0, 0};
  return this->distance(zero) < other.distance(zero);
}

std::ostream &operator<<(std::ostream &os, const Position3D &position) {
  os << "{" << position.x << "," << position.y << "," << position.z;

  return os << "}";
}