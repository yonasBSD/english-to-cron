/// This module provides functionality for processing frequency-related tokens
/// within cron expressions. It defines a function to validate frequency inputs
/// and another to process these inputs, updating the associated `Cron` structure.
///
use super::super::{action::Kind, cron::Cron, stack::Stack};
use regex::Regex;
use std::sync::LazyLock;

static RE_MATCH: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[0-9]+$").unwrap());

/// Checks if the given string is a valid frequency token.
pub fn try_from_token(str: &str) -> bool {
    RE_MATCH.is_match(str)
}

/// Processes the given frequency and updates the specified `Cron` structure.
///
/// This function modifies the `cron` stack based on the provided frequency.
/// If the last item in the stack indicates the start or end of a range,
/// the function updates the corresponding frequency fields. If the stack
/// is empty, it adds a new entry with the specified frequency.
pub fn process(frequency: i32, cron: &mut Cron) {
    if !cron.stack.is_empty() {
        if let Some(last_stack) = cron.stack.last_mut() {
            if last_stack.owner == Kind::RangeEnd {
                last_stack.frequency_end = Some(frequency);
                return;
            } else if last_stack.owner == Kind::RangeStart {
                last_stack.frequency_start = Some(frequency);
                return;
            }
        } else {
            panic!("handle later")
        }
    }
    cron.stack.push(
        Stack::builder(Kind::FrequencyOnly)
            .frequency(frequency)
            .build(),
    );
}
