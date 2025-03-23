use std::str::FromStr;

fn main() {
    let texts = vec![
        "every 15 seconds",
        "every minute",
        "every day at 4:00 pm",
        "at 10:00 am",
        "Run at midnight on the 1st and 15th of the month",
        "on Sunday at 12:00",
    ];

    for text in texts {
        let res = english_to_cron::Cron::from_str(text).unwrap();
        println!("{text}: {res}");
    }
}
