//! This module handles processing of minute-related tokens for cron expressions.
//! It validates input tokens representing minutes and updates the `Cron` structure
//! accordingly.

use super::super::{
    action::Kind,
    cron::Cron,
    stack::{Stack, StartEnd},
};
use regex::Regex;
use std::sync::LazyLock;

/// Regex pattern for matching any form of the word "minute" (including "mins" and "minutes").
/// This pattern is case-insensitive and matches both singular and plural forms.
static RE_MATCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(minutes|minute|mins|min)").unwrap());

/// Regex pattern to specifically match the exact words "minute", "mins", or "minutes".
/// This pattern is case-sensitive and is used to verify if a token is strictly
/// one of the specified minute terms.
static RE_MINUTES: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^(minutes|minute|mins|min)$").unwrap());

/// Checks if the given string is a valid minute token.
pub fn try_from_token(str: &str) -> bool {
    RE_MATCH.is_match(str)
}

/// Processes the given minute token and updates the specified `Cron` structure.
///
/// This function modifies the `cron` stack based on the provided minute token.
/// If the last item in the stack indicates a frequency, the function updates the
/// corresponding minute fields. If a range start or end is detected, it adjusts
/// the minute range accordingly.
pub fn process(token: &str, cron: &mut Cron) {
    if RE_MINUTES.is_match(token) {
        let mut minutes = None;
        if let Some(element) = cron.stack.last_mut() {
            if element.owner == Kind::FrequencyOnly {
                minutes = Some(StartEnd {
                    start: element.frequency,
                    end: None,
                });
                cron.syntax.min = format!("0/{}", element.frequency_to_string());
                cron.stack.pop();
            } else if element.owner == Kind::FrequencyWith {
                minutes = Some(StartEnd {
                    start: element.frequency,
                    end: None,
                });
                cron.syntax.min = element.frequency_to_string();
                cron.stack.pop();
            } else if element.owner == Kind::RangeStart {
                element.min = Some(StartEnd {
                    start: element.frequency_start,
                    end: None,
                });
                return;
            } else if element.owner == Kind::RangeEnd {
                element.min = Some(StartEnd {
                    start: element.frequency_start,
                    end: element.frequency_end,
                });
                element.frequency_end = None;

                if let (Some(frequency_start), Some(frequency_end)) =
                    (element.frequency_start, element.frequency_end)
                {
                    cron.syntax.min = format!("{frequency_start}-{frequency_end}",);
                }

                return;
            }
        }

        if let Some(minutes) = minutes {
            cron.stack
                .push(Stack::builder(Kind::Minute).min(minutes).build());
        }
    }
}
