# Advent of Code 2025
This folder contains solutions to the Advent of Code 2025 challenges.

## Setup
1. This project is written in C++. You'll need:
   - A C++ compiler (GCC, Clang, or MSVC)
   - CMake 3.10 or higher

2. Clone the repository:
    ```sh
    git clone https://github.com/ayodejiige/AdventOfCode.git
    ```

## Building
1. Go to the `2025` directory:
    ```sh
    cd path/to/your/AdventOfCode/2025
    ```

2. Build the project:
    ```sh
    cmake -B build
    cmake --build build
    ```

## Usage
After building, run the solutions for each day's challenge:
```sh
./build/aoc2025 --day <day> --input <input_file>
```

## Project Structure
- `src/` - Source files for each day's solution
- `include/` - Header files
- `inputs/` - Input files for each day
- `external/` - External dependencies (CLI11)