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

rakaly::GameFile parseSave(fs::path filePath, const std::string &input) {
  if (filePath.extension() == ".eu4") {
    return rakaly::parseEu4(input);
  } else if (filePath.extension() == ".ck3") {
    return rakaly::parseCk3(input);
  } else if (filePath.extension() == ".hoi4") {
    return rakaly::parseHoi4(input);
  } else if (filePath.extension() == ".rome") {
    return rakaly::parseImperator(input);
  } else if (filePath.extension() == ".v3") {
    return rakaly::parseVic3(input);
  } else {
    throw std::runtime_error("unrecognized file extension");
  }
}

int main(int argc, const char *argv[]) {
  if (argc != 3) {
    std::cerr << "expected [melt/save] and one file argument\n";
    return 1;
  }

  fs::path filePath = argv[2];
  std::string input = readFile(filePath);

  const auto save = parseSave(filePath, input);
  if (save.is_binary()) {
    std::cerr << "cool! This save is binary!\n";
  }

  if (argv[1] == std::string("meta")) {
    if (auto melt = save.meltMeta()) {
      if (melt->has_unknown_tokens()) {
        std::cerr << "unable to melt all fields\n";
      }

      std::string out;
      melt->writeData(out);
      std::cout << out;
    } else {
      std::cerr << "unable to easily extract meta\n";
    }
  } else if (argv[1] == std::string("save")) {
    auto melt = save.melt();

    if (melt.has_unknown_tokens()) {
      std::cerr << "unable to melt all fields\n";
    }

    // Re-use input buffer in case no work needs to be done
    melt.writeData(input);
    std::cout << input;
  } else {
    throw std::runtime_error("unrecognized command [melt/save]");
  }
}
