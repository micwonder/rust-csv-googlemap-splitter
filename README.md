# CSV File Processor

This project processes a large CSV file by splitting it into multiple smaller files based on a specified number of rows per file. It uses asynchronous file operations and includes progress reporting with a progress bar.

## Features

- Asynchronously reads a large CSV file.
- Splits the CSV file into multiple smaller files.
- Converts coordinates from decimal degrees to degrees, minutes, seconds format.
- Displays a progress bar to track processing progress.

## Prerequisites

- Rust
- Cargo

## Dependencies

- `tokio`: For asynchronous file operations.
- `indicatif`: For displaying a progress bar.

## Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/micwonder/rust-csv-googlemap-splitter.git
   cd rust-csv-googlemap-splitter
