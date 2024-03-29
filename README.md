![ci](https://github.com/rakaly/librakaly/workflows/ci/badge.svg)

# LibRakaly

librakaly is a shared library for [PDX Tools](https://pdx.tools/eu4) functionality that exposes a C and C++ interface and can be embedded in any application that can call C or C++ code.

Below is a whirlwind tour of the C++ API (for more usage, see `sample.cpp`).

```cpp
namespace fs = std::filesystem;

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
  // ... snip getting file path and reading file ...
  const auto save = parseSave(filePath, input);
  if (save.is_binary()) {
    std::cerr << "cool! This save is binary!\n";
  }

  auto melt = save.melt();

  if (melt.has_unknown_tokens()) {
    std::cerr << "unable to melt all fields\n";
  }

  melt.writeData(input);
  std::cout << input;
}
```

## Tutorial

While these steps are linux specific, librakaly can also be used with ease on windows or mac. This tutorial will assume using the prebuilt shared library. If one wants to build librakaly for themselves, see the "Building" section.

- Create a directory to house the tutorial
- [Go to the latest librakaly release](https://github.com/rakaly/librakaly/releases)
- Download the librakaly.so
- Download the rakaly.h
- Download `sample.cpp` for the C++ example. If writing C, one can base their implementation off the C++ wrapper in `./src/cpp_helper.h`
- Invoke gcc as follows

```
g++ sample.cpp -std=c++17 -o melter -O2 librakaly.so
```

- Then invoke the melter like:

```
LD_LIBRARY_PATH=. ./melter save my-ironman-save.eu4
```

## Building

To build:

- Have the [rust toolchain installed](https://rustup.rs/)
- Define an environment variable `EU4_IRONMAN_TOKENS` pointing to a file containing the ironman tokens. More info in the [eu4save repo](https://github.com/rakaly/eu4save#ironman).
- Define an environment variable `CK3_IRONMAN_TOKENS` pointing to a file containing the ironman tokens. More info in the [ck3save repo](https://github.com/rakaly/ck3save#ironman).
- Define an environment variable `IMPERATOR_TOKENS` pointing to a file containing the ironman tokens. More info in the [imperator repo](https://github.com/rakaly/imperator-save#ironman).
- Define an environment variable `HOI4_IRONMAN_TOKENS` pointing to a file containing the ironman tokens. More info in the [hoi4 repo](https://github.com/rakaly/hoi4save#binary-saves).
- Invoke

```
cargo build --release
```
