use std::{collections::BTreeSet, iter::once};

use regex::RegexSet;

use crate::{error::Error, parse::Color};

#[derive(Debug, Clone)]
pub struct Solver {
    pub states: Vec<State>,
    pub exluded_alphas: BTreeSet<char>,
    pub still_possible_alphas: BTreeSet<char>,
}

pub trait FitCells {
    fn fit_cells(&mut self, cells: Vec<(Color, char)>) -> Result<(), Error>;
}

pub trait GenRegexSet {
    fn gen_regex_set(&self) -> Result<RegexSet, Error>;
}

impl GenRegexSet for Solver {
    fn gen_regex_set(&self) -> Result<RegexSet, Error> {
        let excluded_alphas = {
            let filtered = self
                .exluded_alphas
                .iter()
                // yellow/green alpha may turn black because it should only show once/twice/... etc.
                // a better way to handle this case is inference each letter's amount,
                .filter(|alpha| !self.still_possible_alphas.contains(alpha))
                .collect::<String>();
            let word_len = self.states.len();
            if !filtered.is_empty() {
                format!("^[^{}]{{{word_len}}}$", filtered)
            } else {
                format!("^.{{{word_len}}}$")
            }
        };
        let should_exists = self.states.iter().filter_map(|state| {
            if let State::Excluded(alphas) = state {
                Some(itertools::join(alphas, "|"))
            } else {
                None
            }
        });
        let exclude_alphas = self
            .states
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

        Ok(regex_set)
    }
}

impl FitCells for Solver {
    fn fit_cells(&mut self, cells: Vec<(Color, char)>) -> Result<(), Error> {
        if cells.len() != self.states.len() {
            eprintln!("input line length should be 1 and double to word_len!");
            Err(Error::CellLengthMismatch)?;
        }

        for (index, &(color, alpha)) in cells.iter().enumerate() {
            match (color, self.states.get_mut(index).unwrap()) {
                (Color::Black, _) => {
                    self.exluded_alphas.insert(alpha);
                }
                (Color::Green, state @ _) => {
                    *state = State::Ensured(alpha);
                    self.still_possible_alphas.insert(alpha);
                }
                (_, state @ State::Unknown) => match color {
                    Color::Yellow => {
                        *state = State::Excluded(vec![alpha]);
                        self.still_possible_alphas.insert(alpha);
                    }
                    _ => unreachable!(),
                },
                (Color::Yellow, State::Excluded(ref mut ex)) => {
                    ex.push(alpha);
                    self.still_possible_alphas.insert(alpha);
                }
                _ => (),
            }
        }

        Ok(())
    }
}

const INITIAL_STATE: State = State::Unknown;

impl Solver {
    pub fn new(word_len: usize) -> Self {
        Self {
            states: vec![INITIAL_STATE; word_len as usize],
            exluded_alphas: BTreeSet::new(),
            still_possible_alphas: BTreeSet::new(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum State {
    Ensured(char),
    Excluded(Vec<char>),
    #[default]
    Unknown,
}
