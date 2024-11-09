use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

fn get_max<K, V>(map: &HashMap<K, V>) -> Option<&K>
where
    V: Ord,
{
    map.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k)
}

fn parse(file: &str) -> io::Result<()> {
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

    let max = match get_max(&hist) {
        Some(val) => val,
        None => {
            println!("History file is empty!");
            return Ok(());
        }
    };



    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let home_dir = dirs::home_dir().expect("Hey! I can't get your home dir");
    let home_dir = home_dir.to_str().expect("This shouldn't fail");
    let home_zsh_dir = format!("{home_dir}/.zsh_history");

    let result = if args.len() >= 2 {
        parse(&args[1])
    } else if Path::new(".zsh_history").exists() {
        parse(".zsh_history")
    } else if Path::new(&home_zsh_dir).exists() {
        parse(&home_zsh_dir)
    } else {
        println!("Couldn't find history file!");
        println!("Usage: {} [history fike]", args[0]);
        Ok(())
    };

    match result {
        Ok(file) => file,
        Err(error) => panic!("Failed to read hist file: {error}"),
    }
}
