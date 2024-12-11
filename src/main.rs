use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut column: u32 = 0;
    let mut delimiter: char = '\t';
    let mut filename = "";

    for arg in &args[1..] {
        if arg.starts_with("-") {
            if let Some(val) = arg.strip_prefix("-f") {
                if let Ok(num) = val.parse::<u32>() {
                    column = num;
                } else {
                    println!("Invalid value for column: {}", val);
                    return Ok(());
                }
            } else if let Some(val) = arg.strip_prefix("-d") {
                if val.len() == 1 {
                    delimiter = val.chars().next().unwrap();
                } else {
                    println!("Invalid value for delimiter: {}", delimiter);
                    return Ok(());
                }
            } else {
                println!("Unrecognized option: {}", arg);
                return Ok(());
            }
        } else {
            if arg.starts_with("|") {
                break;
            }
            filename = arg;
        }
    }

    if column == 0 {
        println!("No column specified");
        Ok(())
    } else if filename.is_empty() {
        println!("No filename specified");
        Ok(())
    } else {
        output_by_field(filename, column, delimiter)
    }
}

fn output_by_field(filename: &str, column: u32, delimiter: char) -> io::Result<()> {
    let file = File::open(&filename);
    let reader: BufReader<File> = match file {
        Ok(file) => BufReader::new(file),
        Err(_) => {
            println!("Invalid filename: {}", filename);
            return Ok(());
        },
    };

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split(delimiter).collect();

        if column > 0 && column <= fields.len() as u32 {
            // Try to write the result to stdout, and ignore BrokenPipe error
            if let Err(e) = writeln!(io::stdout(), "{}", fields[column as usize - 1]) {
                return if e.kind() == io::ErrorKind::BrokenPipe {
                    Ok(()) // Gracefully exit if pipe is closed
                } else {
                    Err(e) // Propagate other errors
                }
            }
        } else {
            println!("Column {} not found in line", column);
            break;
        }
    }

    Ok(())
}
