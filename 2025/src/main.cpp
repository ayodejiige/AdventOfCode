#include "Dayx.hpp"
#include <CLI/CLI.hpp>
#include <cstdint>
#include <iostream>
#include <memory>
#include <string>

#include <Day1.hpp>

std::unique_ptr<DayX> getDayInstance(uint32_t day,
                                     const std::string &input_file) {
  switch (day) {
  case 1:
    return std::make_unique<Day1>(input_file);
  default:
    throw std::runtime_error("Day not implemented");
  }
}

int main(int argc, char **argv) {
  CLI::App app{"Advent of Code 2025 Solutions"};
  argv = app.ensure_utf8(argv);

  uint32_t day;
  app.add_option("-d,--day", day, "Specify the day to run")->required();

  std::string input_file;
  app.add_option("-i,--input", input_file, "Input file for the specified day")
      ->required();

  CLI11_PARSE(app, argc, argv);

  auto day_x = getDayInstance(day, input_file);

  std::cout << "Part 1: " << day_x->part1() << std::endl;
  std::cout << "Part 2: " << day_x->part2() << std::endl;

  return 0;
}