use std::fs::File;
use std::io::Read;
use std::path::Path;

fn parse(file: &str) {
    let path = Path::new(file);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    println!("{s}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let home_dir = dirs::home_dir().expect("Hey! I can't get your home dir");
    let home_dir = home_dir.to_str().expect("This shouldn't fail");
    let home_zsh_dir = format!("{home_dir}/.zsh_history");

    if args.len() >= 2 {
        parse(&args[1]);
    } else if Path::new(".zsh_history").exists() {
        parse(".zsh_history");
    } else if Path::new(&home_zsh_dir).exists() {
        parse(&home_zsh_dir);
    } else {
        println!("Couldn't find history file!");
        println!("Usage: {} [history fike]", args[0]);
    }
}
