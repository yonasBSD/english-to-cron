//! This module handles processing of hour-related tokens for cron expressions.
//! It validates input tokens representing hours and updates the `Cron` structure
//! accordingly.
//!
use super::super::{
    action::Kind,
    cron::Cron,
    stack::{Stack, StartEnd},
};
use regex::Regex;

lazy_static::lazy_static! {
    /// Regex pattern for matching any form of the word "hour" (including "hrs" and "hours").
    /// This pattern is case-insensitive and matches both singular and plural forms.
    static ref RE_MATCH: Regex = Regex::new(r"(?i)(hour|hrs|hours)").unwrap();
    /// Regex pattern to specifically match the exact words "hour", "hrs", or "hours".
    /// This pattern is case-sensitive and is used to verify if a token is strictly
    /// one of the specified hour terms.
    static ref RE_HOUR: Regex = Regex::new("^(hour|hrs|hours)$").unwrap();

}

/// Checks if the given string is a valid hour token.
pub fn try_from_token(str: &str) -> bool {
    RE_MATCH.is_match(str)
}

/// Processes the given hour token and updates the specified `Cron` structure.
///
/// This function modifies the `cron` stack based on the provided hour token.
/// If the last item in the stack indicates a frequency, the function updates the
/// corresponding hour fields. If a range start or end is detected, it adjusts
/// the hour range accordingly.
pub fn process(token: &str, cron: &mut Cron) {
    if RE_HOUR.is_match(token) {
        let mut hour = None;
        if let Some(element) = cron.stack.last_mut() {
            if element.owner == Kind::FrequencyOnly {
                hour = Some(StartEnd {
                    start: element.frequency,
                    end: None,
                });
                cron.syntax.hour = format!("0/{}", element.frequency_to_string());
                cron.stack.pop();
            } else if element.owner == Kind::FrequencyWith {
                hour = Some(StartEnd {
                    start: element.frequency,
                    end: None,
                });
                cron.syntax.hour = element.frequency_to_string();
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
                    cron.syntax.hour = format!("{frequency_start}-{frequency_end}",);
                }

                return;
            }
        }

        if let Some(hour) = hour {
            cron.stack
                .push(Stack::builder(Kind::Minute).hour(hour).build());
        }
    }
}
