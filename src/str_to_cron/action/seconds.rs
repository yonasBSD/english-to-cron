//! Module for processing second-related tokens in cron expressions.
//!
//! This module interprets tokens that specify seconds, including keywords like
//! "second", "seconds", "sec", and "secs". It updates the `Cron` object with
//! the appropriate values based on the input token.

use super::super::{action::Kind, cron::Cron, stack::Stack};
use regex::Regex;

lazy_static::lazy_static! {
    /// Regular expression to match any form of the word "second".
    static ref RE_MATCH: Regex = Regex::new(r"(?i)(seconds|second|sec|secs)").unwrap();
    /// Regular expression to match exactly the words "second" or "seconds".
    static ref RE_SECUND: Regex = Regex::new("^(seconds|second|sec|secs)$").unwrap();

}

/// Checks if the provided token matches second-related keywords or formats.
pub fn try_from_token(str: &str) -> bool {
    RE_MATCH.is_match(str)
}

/// Processes the provided token to update the cron object with second information.
///
/// This function interprets second-related tokens, updating the `cron` object's
/// syntax seconds based on the provided token. It handles both exact keyword matches
/// and updates the cron stack appropriately.
pub fn process(token: &str, cron: &mut Cron) {
    if RE_SECUND.is_match(token) {
        if let Some(element) = cron.stack.last_mut() {
            if element.owner == Kind::FrequencyOnly {
                cron.syntax.seconds = format!("0/{}", element.frequency_to_string());
                cron.stack.pop();
            } else if element.owner == Kind::FrequencyWith {
                cron.syntax.seconds = element.frequency_to_string();
                cron.stack.pop();
            }
            
        } else {
            cron.syntax.seconds = "*".to_string();
            
        }

        cron.stack.push(Stack::builder(Kind::Secund).build());
    }
}
