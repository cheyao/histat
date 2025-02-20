use clap::Parser;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // History file to read
    #[arg(short, long, default_value_t = String::from("~/.zsh_history"))]
    file: String,

    // Lines to print
    #[arg(short, long, default_value_t = 5)]
    lines: u64,
}

fn parse_normal(file: &str, lines: u64) -> io::Result<()> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut hist: HashMap<String, u64> = HashMap::new();
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                println!("Failed to read a line from hist file: {error}");
                continue;
            }
        };

        let command = line.split_whitespace().next().unwrap_or("");

        hist.entry(command.to_string())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let count_b: BTreeMap<&u64, &String> = hist.iter().map(|(k, v)| (v, k)).collect();

    let mut count: u64 = 0;
    for key in count_b.into_iter().rev() {
        println!("{} {}", key.0, key.1);

        count += 1;
        if count == lines {
            break;
        }
    }

    Ok(())
}

fn parse(file: &str, lines: u64) -> io::Result<()> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut hist: HashMap<String, u64> = HashMap::new();
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                println!("Failed to read a line from hist file: {error}");
                continue;
            }
        };

        let split = line.split(';').collect::<Vec<&str>>();
        let command_part = match split.get(1) {
            Some(line) => line,
            None => continue,
        };
        let command = command_part.split_whitespace().next().unwrap_or("");

        hist.entry(command.to_string())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let count_b: BTreeMap<&u64, &String> = hist.iter().map(|(k, v)| (v, k)).collect();

    let mut count: u64 = 0;
    for key in count_b.into_iter().rev() {
        println!("{} {}", key.0, key.1);

        count += 1;
        if count == lines {
            break;
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    let home_dir = dirs::home_dir().expect("Hey! I can't get your home dir");
    let home_dir = home_dir.to_str().expect("This shouldn't fail");

    let file = args.file.replace("~", home_dir);

    let open_file = File::open(file.clone()).expect("Failed to read history file!");
    let mut reader = BufReader::new(open_file);

    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read history file!");

    let result = if line.starts_with(':') {
        parse(&file, args.lines)
    } else {
        parse_normal(&file, args.lines)
    };

    match result {
        Ok(x) => x,
        Err(error) => panic!("Failed to read hist file: {error}"),
    }
}
