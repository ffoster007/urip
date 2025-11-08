# IPU - URL to IP Address Converter

A simple command-line tool written in Rust that converts URLs to their corresponding IP addresses.

## Features

- Convert any URL to its IP address
- Supports both HTTP and HTTPS URLs
- Fast DNS resolution
- Clean and simple output format

## Installation

### Prerequisites

- Rust and Cargo installed on your system

### Build and Install

1. Clone or navigate to the project directory:
```bash
cd /home/ffoster007/urltoip
```

2. Build the project:
```bash
cargo build --release
```

3. Install the binary:
```bash
cargo install --path .
```

The `ipu` command will be installed to `~/.cargo/bin/ipu` (make sure this directory is in your PATH).

## Usage

Basic syntax:
```bash
ipu <url>
```

### Examples

```bash
# Convert Google's URL to IP
ipu https://www.google.com
# Output: Ip: 142.250.204.100

# Convert GitHub's URL to IP
ipu https://github.com
# Output: Ip: 20.205.243.166

# Convert any website
ipu https://www.example.com
# Output: Ip: 93.184.216.34
```

### Error Handling

The tool will display an error message if:
- The URL format is invalid
- The hostname cannot be resolved
- No arguments are provided

Example error:
```bash
ipu invalid-url
# Output: Error: Invalid URL format
```

## How It Works

1. Parses the input URL
2. Extracts the hostname from the URL
3. Performs DNS resolution to get the IP address
4. Displays the first resolved IP address

## Uninstall

To remove the installed binary:
```bash
cargo uninstall ipu
```
