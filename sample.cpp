#include "rakaly.h"
#include <filesystem>
#include <fstream>
#include <iostream>
#include <string>

namespace fs = std::filesystem;

std::string readFile(fs::path filePath) {
  // lock the file.
  std::ifstream f(filePath, std::ios::in | std::ios::binary);
  const auto sz = fs::file_size(filePath);
  std::string result(sz, '\0');
  f.read(result.data(), sz);
  return result;
}

rakaly::MeltedOutput meltPath(fs::path filePath, const std::string &input) {
  if (filePath.extension() == ".eu4") {
    return rakaly::meltEu4(input);
  } else if (filePath.extension() == ".ck3") {
    return rakaly::meltCk3(input);
  } else if (filePath.extension() == ".hoi4") {
    return rakaly::meltHoi4(input);
  } else if (filePath.extension() == ".rome") {
    return rakaly::meltImperator(input);
  } else if (filePath.extension() == ".v3") {
    return rakaly::meltVic3(input);
  } else {
    throw std::runtime_error("unrecognized file extension");
  }
}

int main(int argc, const char *argv[]) {
  if (argc != 2) {
    std::cerr << "expected one file argument\n";
    return 1;
  }
  fs::path filePath = argv[1];
  std::string input = readFile(filePath);

  const auto melted = meltPath(filePath, input);
  if (melted.was_binary()) {
    std::cerr << "cool! This was converted from binary\n";
    if (melted.has_unknown_tokens()) {
      std::cerr << "but some fields could not be converted\n";
      std::cerr << "and will look like '__unknown_0x' in the output\n";
    }
  }
  melted.writeData(input);
  std::cout << input;
}
