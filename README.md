Sure, here is a sample `README.md` file for your Rust project:

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
   cd csv-file-processor
   ```

2. Add dependencies to `Cargo.toml`:
   ```toml
   [dependencies]
   tokio = { version = "1", features = ["full"] }
   indicatif = "0.17.8"
   ```

3. Build the project:
   ```sh
   cargo build
   ```

## Usage

1. Ensure your input CSV file is placed in the specified directory (e.g., `F:\Super\CSV\Dataset\N039E125.csv`).
2. Run the project:
   ```sh
   cargo run
   ```

## Code Explanation

- `main`: The main asynchronous function that orchestrates reading the CSV file, processing it, and saving the output files.
- `convert_to_dms`: Converts coordinates from decimal degrees to degrees, minutes, seconds format.
- `save_batch_to_file`: Asynchronously saves a batch of lines to a file.
- `count_lines`: Counts the total number of lines in the CSV file for initializing the progress bar.

## Progress Bar

The progress bar displays the progress of reading and processing lines from the CSV file. It provides an estimated time of completion and updates in real-time.

## Example

Suppose you have a CSV file with the following content:
```
29.75 95.3 100
29.76 95.4 101
...
```

The processed output files will be split into smaller files named based on the coordinates, e.g., `dataset\29°0'0"N_95°30'0"W_29°0'0"N_126°30'0"W.txt`.

## Contributing

Feel free to fork the repository and submit pull requests. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
