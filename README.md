![ci](https://github.com/rakaly/librakaly/workflows/ci/badge.svg)

# LibRakaly

librakaly is a shared library for [rakaly](https://rakaly.com/eu4) functionality that exposes a C interface and can be embedded in any application that can call C code.

## Tutorial

While these steps are linux specific, librakaly can also be used with ease on windows or mac. This tutorial will assume using the prebuilt shared library. If one wants to build librakaly for themselves, see the "Building" section.

- Create a directory to house the tutorial
- [Go to the latest librakaly release](https://github.com/rakaly/librakaly/releases)
- Download the librakaly.so
- Download the rakaly.h
- Download the sample C code from the repository
- Invoke gcc as follows

```
gcc sample.c -o melter -O3 librakaly.so
```

- Then invoke the melter like:

```
LD_LIBRARY_PATH=. ./melter my-ironman-save.eu4
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
