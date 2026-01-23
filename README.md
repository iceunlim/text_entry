# text_entry

A simple cross-platform tool to automatically type text using keyboard emulation.

## Features

- **Cross-platform**: Works on Linux, Windows, and macOS
- **Customizable delay**: Adjust the delay between keystrokes
- **Real-time feedback**: Shows progress and statistics during execution
- **Error handling**: Gracefully handles and reports errors
- **Universal binary**: macOS version includes both Intel and Apple Silicon support

## Usage

### Basic Usage

```bash
# Type text with default 20ms delay
text_entry "Hello, world!"

# Type text with custom delay (50ms)
text_entry "Hello, world!" 50
```

### Output Example

```
Input of 'Hello, world!' succeeded. Elapsed time: 369.833625ms
```

## How It Works

1. **Receives input**: Takes text and optional delay as command-line arguments
2. **Initializes keyboard**: Sets up keyboard emulation using the enigo library
3. **Types text**: Simulates keystrokes for each character with specified delay
4. **Reports results**: Shows execution status, timing, and performance metrics

## Build Instructions

### Prerequisites

- **Rust**: Install from [rust-lang.org](https://www.rust-lang.org/)
- **Cross** (for cross-compilation): `cargo install cross`
- **Docker** (for Windows and Linux cross-compilation)
- **Dependencies**: 
  - Linux: X11 development libraries
  - macOS: No additional dependencies
  - Windows: No additional dependencies

### GitHub Actions

The project uses GitHub Actions for automatic building and release:

#### Build Process
- **Trigger events**: Push to main/master branch, pull requests, or manual workflow dispatch
- **Build matrix**: Builds for all supported platforms in parallel
- **Artifacts**: Each platform's build result is uploaded as a separate artifact

#### Release Process
- **Automatic**: Creates a release when a tag is pushed
- **Artifacts**: All build artifacts are attached to the release
- **Versioning**: Uses git tags for version numbers

### Manual Build

To build locally for your current platform:

```bash
# Build in release mode
cargo build --release

# Run the built binary
./target/release/text_entry "Hello, world!"
```

For cross-compilation, you can use the `cross` tool:

```bash
# Install cross
cargo install cross

# Build for Linux x86_64 with musl
cross build --release --target x86_64-unknown-linux-musl
```

## Supported Platforms

| Platform | Architecture | Build Status |
|----------|--------------|-------------|
| Linux    | x86_64       | ✅          |
| Linux    | arm64        | ✅          |
| Windows  | x86 (32-bit) | ✅          |
| macOS    | Universal    | ✅          |

**Note for Windows users**: Only 32-bit version is provided as it's compatible with all Windows systems (including 64-bit Windows). This reduces build complexity and maintenance overhead.

## Use Cases

- **Automated testing**: Fill in forms and text fields automatically
- **Demonstrations**: Showcase typing without manual input
- **Accessibility**: Assist users with motor disabilities
- **Repetitive tasks**: Automate typing of frequently used text
- **Gaming**: Automate in-game chat or commands

## Dependencies

- **enigo**: Cross-platform keyboard emulation library

## License

MIT
