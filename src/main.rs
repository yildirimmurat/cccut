mod utils;

use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;
use utils::clean_non_breaking_spaces;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut columns: Vec<u32> = Vec::new();  // This will hold the columns as u32
    let mut delimiter: char = '\t';
    let mut filename = "-"; // if there is no filename then, stdin

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];

        if arg.starts_with("-") {
            if let Some(val) = arg.strip_prefix("-f") {
                if val.contains(',') {
                    // If columns are comma-separated (e.g., -f1,2,3)
                    for column in val.split(',') {
                        if let Ok(num) = column.trim().parse::<u32>() {
                            columns.push(num);
                        } else {
                            println!("Invalid column number: {}", column);
                            return Ok(());
                        }
                    }
                } else if val.is_empty() {
                    // Expect space-separated columns in the next arguments (e.g., -f 1 2 3)
                    i += 1;  // Move to the next argument where columns are specified
                    if i < args.len() {
                        let next_arg = &args[i];
                        for column in next_arg.split_whitespace() {
                            if let Ok(num) = column.trim().parse::<u32>() {
                                columns.push(num);
                            } else {
                                println!("Invalid column number: {}", column);
                                return Ok(());
                            }
                        }
                    } else {
                        println!("No columns provided after -f");
                        return Ok(());
                    }
                } else {
                    // Single column case (e.g., -f1)
                    if let Ok(num) = val.trim().parse::<u32>() {
                        columns.push(num);
                    } else {
                        println!("Invalid column number: {}", val);
                        return Ok(());
                    }
                }
            } else if let Some(val) = arg.strip_prefix("-d") {
                if val.len() == 1 {
                    delimiter = val.chars().next().unwrap();
                } else {
                    println!("Invalid value for delimiter: {}", delimiter);
                    return Ok(());
                }
            } else if arg != "-" {
                println!("Unrecognized option: {}", arg);
                return Ok(());
            }
        } else {
            if arg.starts_with("|") {
                break;
            }
            filename = arg;
        }
        i += 1;  // Move to the next argument
    }

    if columns.is_empty() {
        println!("No columns specified");
        return Ok(());
    }

    if filename.is_empty() {
        println!("No filename specified");
        return Ok(());
    }

    output_by_field(filename, &columns, delimiter)
}

fn output_by_field(filename: &str, columns: &[u32], delimiter: char) -> io::Result<()> {
    let reader: Box<dyn BufRead> = if filename == "-" {
        Box::new(io::stdin().lock())
    } else {
        let file = File::open(filename)?;
        Box::new(BufReader::new(file))
    };

    for line in reader.lines() {
        let line = line?;
        let cleaned_line = clean_non_breaking_spaces(&line);
        let fields: Vec<&str> = cleaned_line.split(delimiter).collect();

        let mut output = Vec::new();

        for &col in columns {
            if col > 0 && col <= fields.len() as u32 {
                output.push(fields[col as usize - 1]);
            } else {
                println!("Column {} not found in line", col);
                break;
            }
        }

        // Try to write the result to stdout, and ignore BrokenPipe error
        if let Err(e) = writeln!(io::stdout(), "{}", output.join(&delimiter.to_string())) {
            return if e.kind() == io::ErrorKind::BrokenPipe {
                Ok(()) // Gracefully exit if pipe is closed
            } else {
                Err(e) // Propagate other errors
            }
        }
    }

    Ok(())
}

