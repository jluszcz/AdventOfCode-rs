pub mod grid;
pub mod logging;
pub mod math;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

use anyhow::{Result, anyhow};
use clap::Parser;
use log::{LevelFilter, trace};

const INPUT_PATH: &str = "input/input";
const TEST_INPUT_PATH: &str = "input/example";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Test,
    Actual,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "test" => Ok(Self::Test),
            "actual" => Ok(Self::Actual),
            _ => Err(anyhow!("Invalid input type: {}", s)),
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "advent-of-code", version, author, infer_long_args = true)]
struct Args {
    /// increase log level from the default for the input type
    #[arg(short, long)]
    verbose: bool,

    /// input type, Test or Actual
    #[arg(short, long, default_value = "actual")]
    input: String,
}

pub fn init() -> Result<Vec<String>> {
    let args = Args::parse();

    let verbose = args.verbose;
    let input = Input::from_str(&args.input)?;

    let log_level = match (input, verbose) {
        (Input::Actual, false) => LevelFilter::Info,
        (Input::Actual, true) => LevelFilter::Debug,
        (Input::Test, false) => LevelFilter::Debug,
        (Input::Test, true) => LevelFilter::Trace,
    };

    logging::init_logger(log_level)?;

    match input {
        Input::Actual => self::input(),
        Input::Test => test_input(),
    }
}

pub fn init_test() -> Result<Vec<String>> {
    logging::init_test_logger()?;
    test_input()
}

fn input() -> Result<Vec<String>> {
    read_lines(INPUT_PATH)
}

fn test_input() -> Result<Vec<String>> {
    read_lines(TEST_INPUT_PATH)
}

fn read_lines(path: impl AsRef<Path>) -> Result<Vec<String>> {
    let reader = BufReader::new(File::open(path.as_ref())?);
    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        trace!("{}", line);
        lines.push(line);
    }
    Ok(lines)
}
