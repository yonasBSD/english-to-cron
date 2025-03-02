//! Module for processing year-related tokens in cron expressions.
//!
//! This module handles the interpretation of tokens that specify years,
//! including keywords like "year" or "years" and numeric year values.

use super::super::{
    action::Kind,
    cron::Cron,
    stack::{Stack, StartEnd},
    Error, Result,
};
use regex::Regex;
use std::sync::LazyLock;

/// Regular expression to match keywords related to years (e.g., "years", "year") and numeric values.
static RE_MATCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)((years|year)|([0-9]{4}[0-9]*(( ?and)?,? ?))+)").unwrap());

/// Regular expression to match just the keywords for years.
static RE_YEARS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)^(years|year)$").unwrap());

/// Regular expression to match numeric values.
static RE_NUMERIC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[0-9]+").unwrap());

/// Regular expression to validate year format (four digits).
static RE_YEAR_FORMAT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[0-9]{4}$").unwrap());

/// Checks if the provided token matches year-related keywords or formats.
pub fn try_from_token(str: &str) -> bool {
    RE_MATCH.is_match(str)
}

/// Processes the provided token to update the cron object with year information.
///
/// This function interprets year-related tokens, updating the cron's syntax year
/// based on the provided token. It handles both keyword matches and numeric year values.
pub fn process(token: &str, cron: &mut Cron) -> Result<()> {
    if RE_YEARS.is_match(token) {
        cron.syntax.year = "?".to_string();
        if let Some(element) = cron.stack.last_mut() {
            if element.owner == Kind::FrequencyOnly {
                cron.syntax.year = format!("0/{}", element.frequency_to_string());
                cron.stack.pop();
            } else if element.owner == Kind::FrequencyWith {
                cron.syntax.year = element.frequency_to_string();
            } else {
                cron.syntax.year = "*".to_string();
            }
        }
    } else {
        let matches: Vec<_> = RE_NUMERIC.find_iter(token).collect();

        let years: Vec<i32> = matches
            .iter()
            .filter_map(|year| {
                if RE_YEAR_FORMAT.is_match(year.as_str()) {
                    if let Ok(year) = year.as_str().parse::<i32>() {
                        return Some(year);
                    }
                }
                None
            })
            .collect::<Vec<_>>();

        if let Some(element) = cron.stack.last_mut() {
            if element.owner == Kind::RangeStart {
                element.year = Some(element.year.as_ref().map_or_else(
                    || StartEnd {
                        start: years.first().copied(),
                        end: None,
                    },
                    |year| StartEnd {
                        start: years.first().copied(),
                        end: year.end,
                    },
                ));

                return Ok(());
            } else if element.owner == Kind::RangeEnd {
                let year = element.year.as_ref().map_or_else(
                    || StartEnd {
                        start: None,
                        end: years.first().copied(),
                    },
                    |year| StartEnd {
                        start: year.start,
                        end: years.first().copied(),
                    },
                );

                cron.syntax.year = format!(
                    "{}-{}",
                    year.start.unwrap_or_default(),
                    year.end.unwrap_or_default()
                );
                cron.stack.pop();

                return Ok(());
            }
        }
        if years.is_empty() {
            return Err(Error::IncorrectValue {
                state: "year".to_string(),
                error: format!("value {token} is not a year format"),
            });
        }
        cron.syntax.year = String::new();
        for year in years {
            cron.syntax.year = format!("{}{},", cron.syntax.year, year);
        }
        cron.syntax.year = cron.syntax.year.trim_end_matches(',').to_string();
    }

    cron.stack.push(Stack::builder(Kind::Year).build());

    Ok(())
}
