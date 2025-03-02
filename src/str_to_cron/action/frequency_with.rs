//! This file defines functionality for handling frequency-based tokens that include qualifiers
//! such as "3rd" or "5th". These tokens are parsed and processed in relation to their position
//! in a cron syntax structure. The regex patterns help in identifying such tokens, and
//! the `process` function applies the detected frequency to the appropriate cron field.
//!
//! The file is a part of a larger module that converts human-readable strings into cron syntax.
use super::super::{action::Kind, cron::Cron, stack::Stack, Error, Result};
use regex::Regex;
use std::sync::LazyLock;

/// A regex pattern that matches frequency tokens with ordinal suffixes like "th", "nd", "rd", or "st".
static RE_MATCH: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[0-9]+(th|nd|rd|st)$").unwrap());

/// A regex pattern that extracts the numeric prefix of a token, assuming it starts with a number.
static RE_NUMERIC_PREFIX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[0-9]+").unwrap());
/// Checks if a given string token matches the pattern for ordinal-based frequency (e.g., "3rd", "5th").
pub fn try_from_token(str: &str) -> bool {
    RE_MATCH.is_match(str)
}

/// Processes a frequency-based token and applies the corresponding value to the cron syntax structure.
///
/// This function parses the numeric prefix from the token, such as "3" from "3rd", and then
/// updates the cron's internal state based on the token's context (e.g., if it's a range start,
/// range end, or general frequency).
///
/// # Errors
///
/// Returns an error if the token doesn't contain a numeric prefix or if parsing the number fails.
///
pub fn process(token: &str, cron: &mut Cron) -> Result<()> {
    let maybe_numeric_prefix = RE_NUMERIC_PREFIX
        .find(token)
        .ok_or_else(|| Error::Capture {
            state: "frequency_with".to_string(),
            token: token.to_string(),
        })?;
    let frequency =
        maybe_numeric_prefix
            .as_str()
            .parse::<i32>()
            .map_err(|_| Error::ParseToNumber {
                state: "frequency_with".to_string(),
                value: maybe_numeric_prefix.as_str().to_string(),
            })?;

    if let Some(element) = cron.stack.last_mut() {
        if element.owner == Kind::RangeEnd {
            element.frequency_end = Some(frequency);
            return Ok(());
        } else if element.owner == Kind::RangeStart {
            element.frequency_start = Some(frequency);
            return Ok(());
        }
    }

    cron.stack.push(
        Stack::builder(Kind::FrequencyWith)
            .frequency(frequency)
            .day_of_week(cron.syntax.day_of_week.clone())
            .build(),
    );

    Ok(())
}
