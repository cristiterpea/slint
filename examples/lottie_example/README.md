# Slint Lottie Player Example

This example demonstrates a basic Lottie animation player using Slint for the UI and the `rlottie-rs` crate for rendering Lottie animations.

## Structure

-   `lottie_player.slint`: (Located in the repository root) Defines the reusable `LottiePlayer` Slint component and the `Lottie` global API.
-   `main_ui.slint`: Defines the main application window (`MainWindow`) which uses the `LottiePlayer` component.
-   `main.rs`: Contains the Rust backend logic that implements the `Lottie` global API callbacks (loading, rendering, etc.) using `rlottie-rs`.
-   `build.rs`: The build script responsible for compiling the Slint UI files.
-   `Cargo.toml`: Specifies the project dependencies, including `slint`, `rlottie-rs`, and `once_cell`.
-   `test.json`: A sample Lottie animation file (a simple moving red square) used for demonstration.

## Prerequisites

### System Dependencies

This example uses the `rlottie-rs` crate, which is a wrapper around the native `rlottie` C++ library. You will need to have `rlottie` and its dependencies installed on your system for `rlottie-rs` to build and link correctly.

-   **General Requirement**: `pkg-config`, a C++ compiler (like `g++` or `clang`), and `cmake`.
-   **rlottie library**: The `rlottie` library itself.

**Installation (Debian/Ubuntu Example):**

```bash
sudo apt-get update
sudo apt-get install pkg-config cmake g++ librlottie-dev
```

**Installation (Fedora Example):**

```bash
sudo dnf install pkgconfig cmake gcc-c++ rlottie-devel
```

**Installation (macOS Example using Homebrew):**

```bash
brew install pkg-config cmake rlottie
```

For other operating systems or distributions, please refer to the documentation for `rlottie` or `rlottie-rs` for specific installation instructions.

## Building the Example

1.  Navigate to the example directory:
    ```bash
    cd examples/lottie_example
    ```

2.  Build the example using Cargo:
    ```bash
    cargo build
    ```

## Running the Example

After successfully building the example, you can run it using:

```bash
cargo run
```

This should open a window displaying the simple red square Lottie animation defined in `test.json`, moving from left to right and looping.

## Customization

-   To use a different Lottie animation, replace `test.json` with your own Lottie JSON file and update the `source` property in `main_ui.slint`:
    ```slint
    // In main_ui.slint
    LottiePlayer {
        id: player;
        source: "your_animation_file.json"; // Change this line
        // ... other properties
    }
    ```
-   Ensure the new JSON file is placed in the `examples/lottie_example/` directory, or provide an appropriate path.
