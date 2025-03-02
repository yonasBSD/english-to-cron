//! This module provides utilities for processing and validating day-related tokens
//! for use in cron expressions. It leverages regular expressions to match and
//! parse inputs related to weekdays, allowing for flexible input formats.
//!
//! The module defines constants for the days of the week and provides functions
//! to determine whether a given token is valid as a day input, as well as to
//! process that token into a `Cron` structure.

use super::super::{
    action::Kind,
    cron::Cron,
    stack::{Stack, StartEndString},
    Error, Result,
};
use regex::Regex;
use std::sync::LazyLock;

/// Matches various formats for days, including full names and abbreviations.
static RE_MATCH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^((days|day)|(((monday|tuesday|wednesday|thursday|friday|saturday|sunday|WEEKEND|MON|TUE|WED|THU|FRI|SAT|SUN)( ?and)?,? ?)+))$")
        .unwrap()
});

/// Matches the tokens "day" or "days".
static RE_DAY: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)^(day|days)$").unwrap());

/// Matches the abbreviations for weekdays and the term "WEEKEND".
static RE_WEEKDAYS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(MON|TUE|WED|THU|FRI|SAT|SUN|WEEKEND)").unwrap());

// Constant array representing the days of the week in uppercase.
const WEEK_DAYS: [&str; 7] = ["MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];

/// Checks if the provided string matches the expected day token formats.
pub fn try_from_token(str: &str) -> bool {
    RE_MATCH.is_match(str)
}

/// Processes the given token to update the `cron` object with the specified day of the week information.
///
/// This function determines whether the input token specifies days in a "day" or "days" format, or specific weekdays.
/// It then updates the `day_of_week` and `day_of_month` fields in the provided `cron` object based on the matched days.
///
/// # Returns
///
/// * [`Result<()>`] - Returns `Ok(())` if the processing is successful, or an `Error` if the token does not match expected formats.
pub fn process(token: &str, cron: &mut Cron) -> Result<()> {
    if RE_DAY.is_match(token) {
        cron.syntax.day_of_week = "?".to_string();
        if cron.syntax.min == "*" {
            cron.syntax.min = "0".to_string();
        }
        if cron.syntax.hour == "*" {
            cron.syntax.hour = "0".to_string();
        }

        if let Some(element) = cron.stack.last() {
            if element.owner == Kind::FrequencyOnly {
                cron.syntax.day_of_month = format!("*/{}", element.frequency_to_string());
                cron.stack.pop();
            } else if element.owner == Kind::FrequencyWith {
                cron.syntax.day_of_month = element.frequency_to_string();
                cron.stack.pop();
            } else {
                cron.syntax.day_of_month = "*".to_string();
            }
        } else {
            cron.syntax.day_of_month = "*/1".to_string();
        }
    } else {
        let matches: Vec<_> = RE_WEEKDAYS.find_iter(token).collect();
        if matches.is_empty() {
            return Err(Error::IncorrectValue {
                state: "day".to_string(),
                error: format!("value {token} is not a weekend format"),
            });
        }
        cron.syntax.day_of_week = String::new();

        let days: Vec<String> = matches
            .iter()
            .map(|day| day.as_str().to_uppercase())
            .collect::<Vec<_>>();

        if let Some(element) = cron.stack.last_mut() {
            if element.owner == Kind::RangeStart {
                element.day = Some(StartEndString {
                    start: days.first().cloned(),
                    end: element.day.clone().and_then(|a| a.end),
                });
                return Ok(());
            } else if element.owner == Kind::RangeEnd {
                let data = StartEndString {
                    start: element.day.clone().and_then(|a| a.start),
                    end: days.first().cloned(),
                };
                element.day = Some(data.clone());

                if let (Some(start), Some(end)) = (data.start, data.end) {
                    cron.syntax.day_of_week = format!("{start}-{end}",);
                }

                cron.syntax.day_of_month = "?".to_string();
                cron.stack.pop();
                return Ok(());
            }
            cron.stack.pop();
        }

        for &day in &WEEK_DAYS {
            if days.contains(&day.to_string()) && !cron.syntax.day_of_week.contains(day) {
                cron.syntax.day_of_week.push_str(&format!("{day},"));
            }
        }

        // Handle the WEEKEND case
        if days.contains(&"WEEKEND".to_string()) {
            for &day in &["SAT", "SUN"] {
                if !cron.syntax.day_of_week.contains(day) {
                    cron.syntax.day_of_week.push_str(&format!("{day},"));
                }
            }
        }

        cron.syntax.day_of_week = cron.syntax.day_of_week.trim_end_matches(',').to_string();
        cron.syntax.day_of_month = "?".to_string();
    }

    cron.stack.push(
        Stack::builder(Kind::Day)
            .day_of_week(cron.syntax.day_of_week.clone())
            .build(),
    );

    Ok(())
}
