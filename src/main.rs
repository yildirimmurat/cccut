use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

const OPTION_FIELD: &str = "field";

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let (option_type, column, filename) = if args.len() == 3 {
        let option = args[1].trim(); // -f2
        let (option_type, column) = if option.starts_with("-f") {
            (Some(OPTION_FIELD.to_string()), Some(option[2..].to_string()))
        } else {
            println!("Invalid option format: {}", option);
            return Ok(());
        };
        (option_type, column, args[2].clone())
    } else if args.len() == 2 {
        (None, None, args[2].clone())
    } else {
        println!("Usage cccut [option] <filename>");
        return Ok(());
    };

    match option_type {
        Some(option_type) => {
            match option_type.as_str() {
                OPTION_FIELD => {
                    output_by_field(column, filename)
                },
                _ => {
                    println!("Invalid option type: {}", option_type);
                    Ok(())
                },
            }
        },
        None => {
            println!("You must specify a list of bytes, characters, or fields");
            Ok(())
        },
    }
}

fn output_by_field(column: Option<String>, filename: String) -> io::Result<()> {
    let column:u32  = match column {
            Some(c) => match c.parse::<u32>() {
                Ok(num) => num,
            Err(_) => {
                println!("Invalid column number: {}", c);
                return Ok(());
            }
        },
        None => {
            println!("You must specify a column number");
            return Ok(());
        }
    };

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
        let fields: Vec<&str> = line.split('\t').collect();

        if column > 0 && column <= fields.len() as u32 {
            println!("{}", fields[column as usize - 1]);
        } else {
            println!("Column {} not found in line", column);
        }
    }

    Ok(())
}
