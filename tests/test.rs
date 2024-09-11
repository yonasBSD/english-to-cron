use english_to_cron::str_cron_syntax;
use rstest::rstest;

#[rstest]
#[case("Run every 15 second", Ok("0/15 * * * * ? *"))]
#[case("Run every 15 seconds", Ok("0/15 * * * * ? *"))]
#[case("Run every 15 sec", Ok("0/15 * * * * ? *"))]
#[case("Run every 15 secs", Ok("0/15 * * * * ? *"))]
#[case("Every 15 seconds, only on Friday", Ok("0/15 * * ? * FRI *"))]
#[case("every 3 hour", Ok("* 0/3 * * ? *"))]
#[case("Run every 15 minutes", Ok("0/15 * * * ? *"))]
#[case("Run at 10:00 am every day", Ok("0 10 * * ? *"))]
#[case("Run at 12:15 pm every day", Ok("15 12 * * ? *"))]
#[case("Run at 6:00 pm every Monday through Friday", Ok("0 18 ? * MON-FRI *"))]
#[case(
    "Run every 10 minutes Monday through Friday",
    Ok("0/10 * ? * MON-FRI *")
)]
#[case("Every 10 minutes, Monday through Friday", Ok("0/10 * ? * MON-FRI *"))]
#[case(
    "Run every 5 minutes Monday through Friday between 8:00 am and 5:55 pm",
    Ok("0/5 8-17 ? * MON-FRI *")
)]
#[case(
    "Run every 5 minutes Monday through Friday between 8:00 am and 8:00 am",
    Ok("0/5 8-8 ? * MON-FRI *")
)]
#[case(
    "Run every 10 minutes Monday through Friday between 8:00 am and 8:00 pm",
    Ok("0/10 8-20 ? * MON-FRI *")
)]
#[case("7pm every Thursday", Ok("0 19 ? * THU *"))]
#[case("midnight on tuesdays", Ok("0 0 ? * TUE *"))]
#[case("every 5 minutes on Tuesdays", Ok("0/5 * ? * TUE *"))]
#[case("midnight", Ok("0 0 * * ? *"))]
#[case("At 12:00 AM", Ok("0 0 * * ? *"))]
#[case("noon", Ok("0 12 * * ? *"))]
#[case("5:15am every Tuesdays", Ok("15 5 ? * TUE *"))]
#[case("At 05:15 AM, only on Tuesday", Ok("15 5 ? * TUE *"))]
#[case("every day at 17:25", Ok("25 17 * * ? *"))]
#[case(
    "every 3rd day at 2:55 am from January to August in 2019 and 2020",
    Ok("55 2 3 JAN-AUG ? 2019,2020")
)]
#[case(
    "Run every 5 minutes Monday through Friday between 8:00 am and 8:00 pm",
    Ok("0/5 8-20 ? * MON-FRI *")
)]
#[case("Run every second", Ok("* * * * ? *"))]
#[case("Run every 5 seconds", Ok("0/5 * * * * ? *"))]
#[case("Run every minute", Ok("* * * * ? *"))]
#[case("Run every 5 minutes", Ok("0/5 * * * ? *"))]
#[case("Run every hour", Ok("* * * * ? *"))]
#[case("Run every 10 hours", Ok("* 0/10 * * ? *"))]
#[case(
    "Run every 3 minutes between 2:00 pm and 4:00 pm",
    Ok("0/3 14-16 * * ? *")
)]
#[case("Run every 15 minutes on weekdays", Ok("0/15 * * * ? *"))]
#[case("Run every 30 minutes on weekends", Ok("0/30 * ? * SAT,SUN *"))]
#[case("Run every hour only on weekends", Ok("* * ? * SAT,SUN *"))]
#[case("Run every second on the 15th of every month", Ok("* * * 15 ? *"))]
#[case("Every 2 seconds only on Monday", Ok("0/2 * * ? * MON *"))]
#[case(
    "Every 5 minutes, only on the last Friday of the month",
    Ok("0/5 * ? * FRI *")
)]
#[case(
    "Run every 3 hours only on the last day of the month",
    Ok("* 0/3 * * ? *")
)]
#[case("Run every 6 hours, starting at 1:00 pm on Monday", Ok("* 0/6 * *  *"))]
#[case("Run at noon every Sunday", Ok("0 12 ? * SUN *"))]
#[case("Run at 3:45 pm every Friday", Ok("45 15 ? * FRI *"))]
#[case("Run at midnight on the last Sunday of the month", Ok("0 0 ? * SUN *"))]
#[case("Run at noon on the first Monday of January", Ok("0 12 ? JAN MON *"))]
#[case("Run at 8:00 am every weekday", Ok("0 8 * * ? *"))]
#[case("Run at 6:30 pm every day", Ok("30 18 * * ? *"))]
#[case("Run at 9:00 am every weekend", Ok("0 9 ? * SAT,SUN *"))]
#[case(
    "Run at midnight on the 1st and 15th of the month",
    Ok("0 0 1,15 * ? *")
)]
#[case("Run at noon on the last Monday of the month", Ok("0 12 ? * MON *"))]
#[case("Run at 12:45 pm every 2nd day of the month", Ok("45 12 2 * ? *"))]
#[case(
    "Every 5 minutes on the last Thursday of the month",
    Ok("0/5 * ? * THU *")
)]
#[case("Run every 2nd Tuesday of the month", Ok("* * ? * TUE *"))]
#[case("Run every minute from January to March", Ok("* * * JAN-MAR ? *"))]
#[case("Run every 3rd day at noon", Ok("0 12 3 * ? *"))]
#[case("Run every 2nd week at 7:30 pm on Wednesday", Ok("30 19 ? * WED *"))]
#[case(
    "Run at 5:55 am on every first Monday and last Friday of the month",
    Ok("55 5 ? * FRI *")
)]
#[case(
    "Every 4 hours between 9:00 am and 6:00 pm on weekdays",
    Ok("* 9-18 * * ? *")
)]
#[test]
fn can_parse_string(
    #[case] cron_str: &str,
    #[case] expected_result: english_to_cron::Result<&str>,
) {
    let result = str_cron_syntax(cron_str);

    assert_eq!(
        result,
        expected_result
            .clone()
            .map(std::string::ToString::to_string),
        "Failed for input: '{cron_str}'. Expected: {expected_result:?}, Got: {result:?}"
    );
}
