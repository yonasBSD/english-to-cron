mod str_to_cron;
pub use str_to_cron::{Error, Result};

/// Converts an English description of a schedule into cronjob syntax.
///
/// This function takes a natural language description of a recurring schedule
/// (e.g., "Run every 15 seconds", "Run at 6:00 pm every Monday through Friday")
/// and converts it into a valid cron expression that can be used to schedule jobs.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use english_to_cron::str_cron_syntax;
///
/// // Every 15 seconds
/// assert_eq!(str_cron_syntax("Run every 15 seconds").unwrap(), "0/15 * * * * ? *");
///
/// // Every 15 minutes
/// assert_eq!(str_cron_syntax("Run every 15 minutes").unwrap(), "0/15 * * * ? *");
///
/// // Every 15 seconds, but only on Fridays
/// assert_eq!(str_cron_syntax("Every 15 seconds, only on Friday").unwrap(), "0/15 * * ? * FRI *");
///
/// // At 6:00 PM from Monday to Friday
/// assert_eq!(str_cron_syntax("Run at 6:00 pm every Monday through Friday").unwrap(), "0 18 ? * MON-FRI *");
///
/// // Every 3rd day at 2:55 AM between January and August in 2019 and 2020
/// assert_eq!(str_cron_syntax("every 3rd day at 2:55 am from January to August in 2019 and 2020").unwrap(), "55 2 3 JAN-AUG ? 2019,2020");
/// ```
///
/// # Errors
///
/// This function returns an [`Error`] if it is unable to parse the provided string
/// into a valid cron syntax. This may occur when the input is incomplete, ambiguous,
/// or does not follow a recognizable pattern.
///
/// # Return
///
/// Returns a [`Result`] containing the parsed cron expression as a `String` on success,
/// or an [`Error`] if parsing fails.
///
/// [`Error`]: str_to_cron::Error
pub fn str_cron_syntax(input: &str) -> str_to_cron::Result<String> {
    let tokenizer = str_to_cron::Tokenizer::new();
    let tokens = tokenizer.run(input);

    if tokens.is_empty() {
        return Err(str_to_cron::Error::InvalidInput);
    }

    str_to_cron::to_string(tokens)
}
