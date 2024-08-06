use pyo3::prelude::*;
use clap::Parser;
use lambda_calculus::*;
use std::fmt::*;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

/// Simulation analysis
mod analysis;

/// Global configuration
mod config;

/// Random expression generators
mod generators;

/// Main AlChemy simulation module
mod soup;

/// From example
/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

fn read_lines_from_file(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    reader.lines().collect()  // Note: no semicolon here
}
#[pyfunction]
fn run_alchemy(expressions_strings: Vec<String>, config_file: &str) -> PyResult<Vec<String>> {
    // Generate the configuration 
    let contents = read_to_string(config_file)?;
    let cfg = config::Config::from_config_str(&contents);
    // Build soup from configuration
    let mut soup = soup::Soup::from_config(&cfg);
    soup.set_limit(cfg.reduction_cutoff);
    // Add expressions to soup
    // let expressions_strings = read_lines_from_file(exprs_file)?;
    let expressions = expressions_strings
        .iter()
        .map(|s| parse(s, Classic).unwrap());
    soup.perturb(expressions);

    // Set some configuration parameters
    let limit = cfg.run_limit;
    let log = cfg.print_reaction_results;
    // Run simulations
    soup.simulate_for(limit, log);
    // Get the final expressions
    let end_exprs: Vec<String> = soup.expressions()
        .into_iter()
        .map(|expr| expr.to_string())
        .collect();
    // Return
    Ok(end_exprs)
}


/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn alchemy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?);
    m.add_function(wrap_pyfunction!(run_alchemy, m)?)
}