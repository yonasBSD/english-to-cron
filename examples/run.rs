fn main() {
    let texts = vec![
        "Run every second",
        "Run every minute",
        "Run every 5 minutes",
        "Run every hour",
        "Run every 3 minutes between 2:00 pm and 4:00 pm",
        "Run every 15 minutes on weekdays",
        "Run every minute from January to March",
        "Run every 3rd day at noon",
    ];

    for text in texts {
        let res = english_to_cron::str_cron_syntax(text);

        println!("{text}: {res:#?}");
    }
}
