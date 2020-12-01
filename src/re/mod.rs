use regex::Regex;
use std::ops::Deref;
use std::str::FromStr;

#[cfg(test)]
mod test;

/// Wrapper around Regex for PartialEq in testing
#[derive(Debug)]
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
