use std::error::Error;
use std::process;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let mut calories = get_sums(&contents);

    let most_calories = calories.iter().max().unwrap();
    println!("Most calories: {most_calories}");

    calories.sort();
    let top_three_sum: u32 = calories.iter().rev().take(3).sum();
    println!("Top 3 calories sum: {top_three_sum}");

    Ok(())
}

fn get_sums(contents: &str) -> Vec<u32> {
    let mut results: Vec<u32> = Vec::new();
    let mut result: u32 = 0;

    for line in contents.lines() {
        if line.trim().is_empty() {
            results.push(result);
            result = 0;
        } else {
            result += line.parse::<u32>().expect("Not a number!");
        }
    }

    results
}

struct Config {
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}
