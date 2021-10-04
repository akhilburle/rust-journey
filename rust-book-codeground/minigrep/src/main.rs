use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    file: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() != 3 {
            return Err(String::from("Expected exactly 2 arguments"));
        }
        Ok(Config {
            query: args[1].clone(),
            file: args[2].clone(),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Couldn't parse arguments: {}", err);
        process::exit(1);
    });
    println!(
        "\n❓Query: \"{}\"\n📁File: \"{}\"",
        config.query, config.file
    );

    let contents = fs::read_to_string(&config.file)
        .expect(&format!("Failed to read the contents of {}", &config.file)[..]);

    println!("\n📖Contents:\n\n{file_contents}", file_contents = contents);
}
