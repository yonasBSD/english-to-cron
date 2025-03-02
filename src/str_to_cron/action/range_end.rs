/// Module for processing range-related tokens in cron expressions.
///
/// This module handles the interpretation of tokens that represent ranges or connections
/// between elements in cron scheduling, such as "to", "through", "ending", and "and".
use super::super::{action::Kind, cron::Cron, stack::StartEndString};
use regex::Regex;
use std::sync::LazyLock;

/// Regular expression to match range-related keywords (e.g., "to", "through").
static RE_MATCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(to|through|ending|end|and)").unwrap());

/// Checks if the provided token matches range-related keywords.
pub fn try_from_token(str: &str) -> bool {
    RE_MATCH.is_match(str)
}

/// Processes the cron object to interpret range-related tokens.
pub fn process(cron: &mut Cron) {
    if let Some(element) = cron.stack.last_mut() {
        match element.owner {
            Kind::FrequencyWith | Kind::FrequencyOnly => {
                element.frequency_start = element.frequency;
            }

            Kind::Day => {
                element.day = match &element.day {
                    Some(day) => Some(StartEndString {
                        start: element.day_of_week.clone(),
                        end: day.end.clone(),
                    }),
                    None => Some(StartEndString {
                        start: element.day_of_week.clone(),
                        end: None,
                    }),
                };
            }

            Kind::Month => {
                element.owner = Kind::RangeEnd;
            }

            Kind::RangeStart => element.owner = Kind::RangeEnd,
            Kind::Year
            | Kind::ClockTime
            | Kind::Minute
            | Kind::Hour
            | Kind::RangeEnd
            | Kind::Secund => {}
        }
        element.owner = Kind::RangeEnd;
    }
}
