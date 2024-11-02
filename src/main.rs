use std::{collections::BTreeSet, iter::once};

use std::io::Write;

use anyhow::Result;
use clap::Parser;
use regex::bytes::RegexSet;
use wordle_solver::{
    arg,
    parse::{self, Color},
};

fn main() -> Result<()> {
    let arg::Args {
        dict_file,
        word_len,
    } = arg::Args::parse();
    let dict = std::fs::read_to_string(dict_file)?;

    const INITIAL_STATE: State = State::Unknown;
    let mut states = vec![INITIAL_STATE; word_len as usize];
    let mut exluded_alphas = BTreeSet::new();
    let mut still_possible_alphas = BTreeSet::new();

    let mut lines = std::io::stdin().lines();
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
            Ok(cell) => cell,
        };

        if cells.len() != word_len as usize {
            eprintln!("input line length should be 1 and double to word_len!");
            continue;
        }

        for (index, &(color, alpha)) in cells.iter().enumerate() {
            match (color, states.get_mut(index).unwrap()) {
                (Color::Black, _) => {
                    exluded_alphas.insert(alpha);
                }
                (Color::Green, state @ _) => {
                    *state = State::Ensured(alpha);
                    still_possible_alphas.insert(alpha);
                }
                (_, state @ State::Unknown) => match color {
                    Color::Yellow => {
                        *state = State::Excluded(vec![alpha]);
                        still_possible_alphas.insert(alpha);
                    }
                    _ => unreachable!(),
                },
                (Color::Yellow, State::Excluded(ref mut ex)) => {
                    ex.push(alpha);
                    still_possible_alphas.insert(alpha);
                }
                _ => (),
            }
        }

        let excluded_alphas = {
            let filtered = exluded_alphas
                .iter()
                // yellow/green alpha may turn black because it should only show once/twice/... etc.
                // a better way to handle this case is inference each letter's amount,
                .filter(|alpha| !still_possible_alphas.contains(alpha))
                .collect::<String>();
            if !filtered.is_empty() {
                format!("^[^{}]{{{word_len}}}$", filtered)
            } else {
                format!("^.{{{word_len}}}$")
            }
        };
        let should_exists = states.iter().filter_map(|state| {
            if let State::Excluded(alphas) = state {
                Some(itertools::join(alphas, "|"))
            } else {
                None
            }
        });
        let exclude_alphas = states
            .iter()
            .map(|s| match s {
                State::Ensured(alpha) => alpha.to_string(),
                State::Excluded(excluded) => format!("[^{}]", excluded.iter().collect::<String>()),
                State::Unknown => ".".to_string(),
            })
            .collect::<String>();
        let regex_set = RegexSet::new(
            once(excluded_alphas)
                .chain(should_exists)
                .chain(once(exclude_alphas)),
        )?;
        dbg!(regex_set.patterns());
        let mut lock = std::io::stdout().lock();
        writeln!(&mut lock, "current result: \n--- start of result ---")?;
        for word in dict
            .lines()
            .filter(|word| regex_set.matches(word.as_bytes()).iter().count() == regex_set.len())
        {
            writeln!(&mut lock, "{word}")?;
        }
        writeln!(&mut lock, "--- end of result ---")?;
    }

    Ok(())
}

#[derive(Clone, Debug, Default)]
pub enum State {
    Ensured(char),
    Excluded(Vec<char>),
    #[default]
    Unknown,
}
