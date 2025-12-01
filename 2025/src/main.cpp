#include "DayX.hpp"
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

  uint32_t day = 1;
  app.add_option("-d,--day", day, "Specify the day to run (default: 1)");

  std::string input_directory = "./inputs";
  app.add_option("-i,--input-dir", input_directory,
                 "Directory containing input files (default: ./inputs)");

  CLI11_PARSE(app, argc, argv);

  std::string input_file =
      input_directory + "/day" + std::to_string(day) + ".txt";

  try {
    auto day_x = getDayInstance(day, input_file);
    day_x->solve();

    std::cout << "=== Day " << day << " ===" << std::endl;
    std::cout << "Part 1: " << day_x->part1() << std::endl;
    std::cout << "Part 2: " << day_x->part2() << std::endl;
  } catch (const std::exception &e) {
    std::cerr << "Error: " << e.what() << std::endl;
    return 1;
  }

  return 0;
}