//! Module for processing range start-related tokens in cron expressions.

use super::super::{action::Kind, cron::Cron, stack::Stack};
use regex::Regex;

lazy_static::lazy_static! {
    /// Regular expression to match keywords indicating the start of a range (e.g., "between", "starting").
    static ref RE_MATCH: Regex = Regex::new(r"(?i)(between|starting|start)").unwrap();
}

/// Checks if the provided token matches range start-related keywords.
pub fn try_from_token(str: &str) -> bool {
    RE_MATCH.is_match(str)
}

/// Processes the cron object to interpret range start-related tokens.
pub fn process(cron: &mut Cron) {
    cron.stack.push(Stack::builder(Kind::RangeStart).build());
}
