use regex::Regex;

lazy_static::lazy_static! {
    static ref RE_TOKENS: Regex = Regex::new(r"(?i)(?:seconds|second|secs|sec)|(?:hours?|hrs?)|(?:minutes?|mins?|min)|(?:months?|(?:january|february|march|april|may|june|july|august|september|october|november|december|jan|feb|mar|apr|may|jun|jul|aug|sept|oct|nov|dec)(?: ?and)?,? ?)+|[0-9]+(?:th|nd|rd|st)|(?:[0-9]+:)?[0-9]+ ?(?:am|pm)|[0-9]+:[0-9]+|(?:noon|midnight)|(?:days?|(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|weekend|mon|tue|wed|thu|fri|sat|sun)(?: ?and)?,? ?)+|(?:[0-9]{4}[0-9]*(?: ?and)?,? ?)+|[0-9]+|(?:to|through|ending|end|and)|(?:between|starting|start)").unwrap();
}

pub struct Tokenizer {
    regex: Regex,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            regex: RE_TOKENS.clone(),
        }
    }

    pub fn run(&self, input_string: &str) -> Vec<String> {
        let matches = self
            .regex
            .find_iter(input_string)
            .map(|m| m.as_str().trim().to_string())
            .collect();

        matches
    }
}
