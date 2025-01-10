# File Compression Utility

A high-performance, user-friendly file compression utility written in Rust that provides efficient compression with real-time progress tracking and detailed statistics.

## Features

This utility offers a robust set of features for file compression:

- Multiple compression levels (fast, default, best) to balance speed and compression ratio
- Real-time progress tracking with estimated time remaining
- Detailed compression statistics including file sizes and compression ratios
- Colored terminal output for enhanced readability
- Comprehensive error handling and user feedback
- Command-line interface with intuitive options

## Installation

### Prerequisites

- Rust 1.70.0 or higher
- Cargo package manager

### Building from Source

1. Clone the repository:

```bash
git clone https://github.com/kushwahramkumar2003/compression-utility.git
cd compression-utility
```

2. Build the project:

```bash
cargo build --release
```

The compiled binary will be available in `target/release/compression-utility`.

## Usage

### Basic Compression

To compress a file using default settings:

```bash
compression-utility input.txt output.gz
```

### Advanced Options

The utility supports several command-line options:

```bash
compression-utility [OPTIONS] <SOURCE> <TARGET>

ARGS:
    <SOURCE>    Source file to compress
    <TARGET>    Target compressed file

OPTIONS:
    -c, --compression <LEVEL>    Compression level (fast, default, best)
    -q, --quiet                  Disable progress bar
    -h, --help                   Print help information
    -V, --version                Print version information
```

### Examples

Compress with maximum compression:

```bash
compression-utility --compression best large-file.txt compressed.gz
```

Compress without progress bar:

```bash
compression-utility --quiet document.txt document.gz
```

## Performance

The utility is optimized for both speed and compression ratio:

- Uses efficient buffered I/O operations
- Provides multiple compression levels for different use cases
- Minimal memory footprint
- Progress tracking with minimal overhead

## Project Structure

```
compression-utility/
├── src/
│   └── main.rs              # Main application code
├── Cargo.toml              # Project dependencies and metadata
├── Cargo.lock              # Lock file for dependencies
└── README.md              # This file
```

## Dependencies

The project relies on the following Rust crates:

```toml
[dependencies]
flate2 = "1.0"            # Compression algorithm implementation
clap = "3.0"              # Command-line argument parsing
colored = "2.0"           # Terminal colors and formatting
indicatif = "0.17"        # Progress bar and indicators
```

## Error Handling

The utility provides clear error messages for common scenarios:

- Missing or invalid input files
- Permission issues
- Disk space problems
- Invalid compression options

## Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

Please ensure your PR includes:

- A clear description of the changes
- Updated documentation if necessary
- Any new tests for added functionality

## Future Enhancements

Features planned for future releases:

- Multiple file compression support
- Directory compression
- Additional compression formats (ZIP, BZIP2)
- Decompression functionality
- File integrity verification
- Parallel compression for large files

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The Rust community for excellent documentation and crates
- Contributors to the core compression libraries
- Everyone who has submitted issues and pull requests

## Contact

Ramkumar kushwah - [@ramkumar_9301](https://twitter.com/ramkumar_9301) - kushwahramkumar2003@gmail.com

Project Link: [https://github.com/kushwahramkumar2003/compression-utility](https://github.com/yourusername/compression-utility)
