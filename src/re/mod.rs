use regex::{Captures, Regex};
use std::ops::Deref;
use std::str::FromStr;

mod parse;
#[cfg(test)]
mod test;

/// Wrapper around Regex for PartialEq in testing
#[derive(Debug, Clone)]
pub struct Re {
    #[cfg(test)]
    content: String,
    regex: Regex,
}

impl FromStr for Re {
    type Err = regex::Error;

    #[cfg(test)]
    fn from_str(s: &str) -> Result<Re, regex::Error> {
        let regex = Regex::from_str(s)?;
        let content = s.to_string();
        Ok(Re { regex, content })
    }

    #[cfg(not(test))]
    fn from_str(s: &str) -> Result<Re, regex::Error> {
        let regex = Regex::from_str(s)?;
        Ok(Re { regex })
    }
}

impl Deref for Re {
    type Target = Regex;

    fn deref(&self) -> &Regex {
        &self.regex
    }
}

#[cfg(test)]
impl PartialEq for Re {
    fn eq(&self, other: &Self) -> bool {
        self.content.eq(&other.content)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Pat {
    Replay,
    Expansion(Vec<Expansion>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expansion {
    Lit(String),
    Whole,
    Pos(usize),
}

impl Pat {
    pub fn expand(&self, captures: &Captures) -> String {
        if let Pat::Expansion(exps) = self {
            let mut buf = String::new();

            for exp in exps {
                match exp {
                    Expansion::Lit(lit) => buf.push_str(lit),
                    Expansion::Whole => buf.push_str(&captures[0]),
                    Expansion::Pos(i) => buf.push_str(&captures[*i]),
                }
            }

            buf
        } else {
            unreachable!("replay should never actually expand");
        }
    }

    pub fn compatible(&self, regex: &Regex) -> bool {
        regex.captures_len() > self.max_pos()
    }

    pub fn max_pos(&self) -> usize {
        match self {
            Pat::Replay => 0,
            Pat::Expansion(v) => {
                let mut m = 0;

                for exp in v {
                    match exp {
                        Expansion::Pos(p) if *p > m => m = *p,
                        _ => (),
                    };
                }

                m
            }
        }
    }
}
