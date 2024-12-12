# RABBITX

RABBITX is a command-line tool developed in Rust, designed to search for secrets (sensitive information such as passwords or tokens) within files in a directory.

## Requirements

The project has only one dependency:

- [Regex](https://crates.io/crates/regex) (regular expression handling) - used to identify patterns that may contain secrets.

## Installation

To build the project, you will need Rust and Cargo installed on your machine. If you haven't installed them yet, follow the instructions [here](https://www.rust-lang.org/tools/install).

### Build

1. Clone the repository:

    ```bash
    git clone https://github.com/your-username/rabbitx.git
    cd rabbitx
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

3. The binary will be generated in the `target/release` directory.

## Usage

RABBITX requires superuser permissions to run correctly. To execute the tool in the current directory, use the following command:

```bash
sudo ./target/release/rabbitx
```

This command will search for patterns that may indicate the presence of secrets in all files within the current directory and its subdirectories.

`If you want to use it in a specific directory, you will need to move the binary to the desired directory.`

## Contributing

If you encounter any issues or would like to contribute with improvements, feel free to open an issue or submit a pull request.