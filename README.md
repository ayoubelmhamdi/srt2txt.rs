## SRT to TXT Converter

This Rust program takes an `SRT` (SubRip Subtitle) file as input and converts it to a clean `TXT` format, where the lines are grouped by their corresponding timestamps.

### Features

- Removes unwanted patterns and lines (e.g., lines containing '[Musique]', '</c>', '<00:xxxxxx><c>', empty lines, and lines with only numbers)
- Groups the remaining lines by their timestamps, separating each group with a blank line
- Writes the processed content to a new TXT file with the same name as the input SRT file

### Usage

```bash
git clone https://github.com/ayoubelmhamdi/srt2txt.rs.git
cd srt2txt
cargo build --release
./target/release/srt2txt file.srt
```

This will create a new file named `file.srt.txt` with the processed content.

### Example

Input SRT file:

```
1
00:00:00,000 --> 00:00:05,000
This is the
first line.

2
00:00:05,000 --> 00:00:10,000
This is the second line.

[Musique]
```

Output TXT file:

```
This is the first line.

This is the second line.
```

### Dependencies

- [regex](https://crates.io/crates/regex): Used for pattern matching and removal.

### Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.

### License

This project is licensed under the [MIT License](LICENSE).
