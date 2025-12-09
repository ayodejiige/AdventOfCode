// Debugging Utilities
#include <iostream>
#include <map>

// Pair printing
template <typename T1, typename T2>
std::ostream &operator<<(std::ostream &os, const std::pair<T1, T2> &p) {
  return os << "(" << p.first << "," << p.second << ")";
}

template <typename T> struct is_container : std::false_type {};
template <typename T> struct is_container<std::vector<T>> : std::true_type {};

// Container printing
template <typename Container>
typename std::enable_if<is_container<Container>::value, std::ostream &>::type
operator<<(std::ostream &os, const Container &container) {
  os << "[";
  bool first = true;
  for (const auto &item : container) {
    if (!first)
      os << ", ";
    os << item;
    first = false;
  }
  return os << "]";
}

// Map printing
template <typename K, typename V>
std::ostream &operator<<(std::ostream &os, const std::map<K, V> &map) {
  os << "{";
  bool first = true;
  for (const auto &[key, value] : map) {
    if (!first)
      os << ", ";
    os << key << ": " << value;
    first = false;
  }
  return os << "}";
}