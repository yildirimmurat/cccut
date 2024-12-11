# Clone Unix Cut Tool

This is a clone unix cut tool

The tool is used through the command line with the command `cccut`

## Steps to Create and Use the Tool

### 1. Compile the Program

First, build the program by running the following command:

```bash
cargo build --release
```

### 2. Move the executable to a directory in your PATH

To make the tool accessible from anywhere in your terminal, move the compiled executables to a directory that is included in your system's PATH.

```bash
sudo cp target/release/cccut /usr/local/bin/
```

### 3. Verify the installation

To verify the installation, check if the executable is accessible from anywhere in your terminal:

```bash
which cccut
```


### 4. Usage

```bash
cccut [options] <filename>
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributions

Contributions are welcome! Feel free to open issues or submit pull requests to improve the tool. To contribute:

1. Fork the repository.
2. Create a new branch.
3. Make your changes.
4. Submit a pull request.
