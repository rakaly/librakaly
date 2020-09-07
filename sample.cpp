#include "rakaly_wrapper.h"
#include <filesystem>
#include <fstream>
#include <iostream>
#include <string>

namespace fs = std::filesystem;

std::string readFile(fs::path path) {
  // lock the file.
  std::ifstream f(path, std::ios::in | std::ios::binary);
  const auto sz = fs::file_size(path);
  std::string result(sz, '\0');
  f.read(result.data(), sz);
  return result;
}

int main(int argc, const char *argv[]) {
  if (argc != 2) {
    fprintf(stderr, "expected one ironman file argument\n");
    return 1;
  }
  std::string input = readFile(argv[1]);
  std::string out = rakaly::meltEU4(input);
  std::cout << out;
}
