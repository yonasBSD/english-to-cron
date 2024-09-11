# English to CronJob Syntax Converter
[![Crates.io](https://img.shields.io/crates/v/english-to-cron.svg)](https://crates.io/crates/english-to-cron)
[![Docs.rs](https://docs.rs/english-to-cron/badge.svg)](https://docs.rs/english-to-cron)

This project is inspired by the library natural-cron, which converts natural language into cron expressions. `english-to-cron` brings similar functionality to the Rust ecosystem, allowing developers to easily schedule cron jobs using English text.


## Features

- Converts various English text descriptions into cron job syntax.
- Supports complex patterns including specific days, time ranges, and more.
- Handles multiple time formats including AM/PM and 24-hour notation.

## Installation

Add the following line to your `Cargo.toml` under `[dependencies]`:

```toml
english_to_cron = "0.1.0" 
```

## Usage
Simply provide an English phrase describing the schedule, and the library will return the corresponding cron job syntax.
```rust
use english_to_cron::str_cron_syntax;

fn main() {
    // Example: every 15 seconds
    let cron_syntax = str_cron_syntax("Run every 15 seconds").unwrap();
    assert_eq!(cron_syntax, "0/15 * * * * ? *");
    
    // Example: every Monday at 6:00 pm
    let cron_syntax = str_cron_syntax("Run at 6:00 pm every Monday through Friday").unwrap();
    assert_eq!(cron_syntax, "0 18 ? * MON-FRI *");
}
```

## Full List of Supported English Patterns

| English Phrase                                                   	| CronJob Syntax             	|
|------------------------------------------------------------------	|----------------------------	|
| Run every 15 seconds                                             	| 0/15 * * * * ? *           	|
| Run every 15 sec                                                 	| 0/15 * * * * ? *           	|
| Every 15 seconds, only on Friday                                 	| 0/15 * * ? * FRI *         	|
| Run every 3 hours                                                	| * 0/3 * * ? *              	|
| Run at 10:00 am every day                                        	| 0 10 * * ? *               	|
| Run at 6:00 pm every Monday through Friday                       	| 0 18 ? * MON-FRI *         	|
| Run every 10 minutes Monday through Friday                       	| 0/10 * ? * MON-FRI *       	|
| 7pm every Thursday                                               	| 0 19 ? * THU *             	|
| midnight on Tuesdays                                             	| 0 0 ? * TUE *              	|
| every day at 17:25                                               	| 25 17 * * ? *              	|
| every 3rd day at 2:55 am from January to August in 2019 and 2020 	| 55 2 3 JAN-AUG ? 2019,2020 	|

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests to help improve the library.