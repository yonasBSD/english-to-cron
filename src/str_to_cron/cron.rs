use super::{action, stack::Stack, Result};

#[derive(Default, Debug)]
pub struct Cron {
    pub syntax: Syntax,
    pub stack: Vec<Stack>,
}

#[derive(Debug)]
pub struct Syntax {
    pub seconds: String,
    pub min: String,
    pub hour: String,
    pub day_of_month: String,
    pub day_of_week: String,
    pub month: String,
    pub year: String,
}

impl Default for Syntax {
    fn default() -> Self {
        Self {
            seconds: "0".to_string(),
            min: "*".to_string(),
            hour: "*".to_string(),
            day_of_month: "*".to_string(),
            day_of_week: "?".to_string(),
            month: "*".to_string(),
            year: "*".to_string(),
        }
    }
}

impl std::fmt::Display for Cron {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {}",
            self.syntax.seconds,
            self.syntax.min,
            self.syntax.hour,
            self.syntax.day_of_month,
            self.syntax.month,
            self.syntax.day_of_week,
            self.syntax.year,
        )
    }
}

pub fn to_string(tokens: Vec<String>) -> Result<String> {
    let mut cron = Cron::default();
    for token in tokens {
        if let Some(state) = action::try_from_token(&token) {
            state.process(&token, &mut cron)?;
        }
    }

    Ok(format!("{cron}"))
}
