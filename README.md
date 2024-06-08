# C + Rust

A little `Rust` with your `C/C++`. The *"Hello world"* to building `Rust` libs for `C/C++` projects with CMake.

## Prerequisites

The easiest way to get started is by using the Visual Studio Code Remote - Containers / Codespaces [development container](.devcontainer/devcontainer.json) included in this repository. This container comes with Rust, Cargo, and several VSCode extensions pre-installed to help you get started quickly.

- For [Remote - Containers](https://aka.ms/vscode-remote/download/containers), use the **Remote-Containers: Open Repository in Container...** command which creates a Docker volume for better disk I/O on macOS and Windows.
- For Codespaces, install the [GitHub Codespaces](https://marketplace.visualstudio.com/items?itemName=GitHub.codespaces) extension in VSCode, and use the **Codespaces: Create New Codespace command**.

Once your workspace is setup, open a terminal to check everything is working:

```bash
cargo --version
```

```bash
rustc --version
```

## Building

The easiest way to build the project is using the CMake extension in VSCode (included in the devcontainer). Open the command palette and run the **CMake: Configure** command, then the **CMake: Build** command.

Alternatively, you can build the project from the terminal:

```bash
mkdir build && cd build
cmake ..
cmake --build .
```

This will build the `C` executable and the `Rust` library. From CMake, cargo is invoked by [corrosion](https://github.com/corrosion-rs/corrosion) to build the Rust library. The Rust project includes a build script that will generate a C header file using [cbindgen](https://github.com/mozilla/cbindgen). This header file is included in the C project to provide the necessary declarations for the Rust functions.

> Note: the first time you build the project, CMake will download the `Rust` dependencies and build the `Rust` library. This may take a few minutes.

## Running

Once the project is built, you can run the `C` executable from the terminal:

```bash
./build/c_plus_rust
```
