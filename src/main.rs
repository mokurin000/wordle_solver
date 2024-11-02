use std::io::Write;

use anyhow::Result;
use clap::Parser;
use wordle_solver::solver::{FitCells, GenRegexSet, Solver};
use wordle_solver::{
    arg,
    parse::{self},
};

fn main() -> Result<()> {
    let arg::Args {
        dict_file,
        word_len,
    } = arg::Args::parse();
    let dict = std::fs::read_to_string(dict_file)?;

    let mut lines = std::io::stdin().lines();
    let mut solver = Solver::new(word_len as _);

    while let Some(Ok(line)) = lines.next() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let cells = match parse::parse_line(&line) {
            Err(e) => {
                eprintln!("error: {e}"); // validate input
                continue;
            }
            Ok(cells) => cells,
        };

        solver.fit_cells(cells)?;
        let regex_set = solver.gen_regex_set()?;

        dbg!(regex_set.patterns());
        let mut lock = std::io::stdout().lock();
        writeln!(&mut lock, "current result: \n--- start of result ---")?;
        for word in dict
            .lines()
            .filter(|word| regex_set.matches(word).iter().count() == regex_set.len())
        {
            writeln!(&mut lock, "{word}")?;
        }
        writeln!(&mut lock, "--- end of result ---")?;
    }

    Ok(())
}
