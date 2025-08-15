# e9571_time_number Usage Examples

This document provides examples of using the `e9571_time_number` module functions in a Rust program, tailored for casino scenarios such as generating timestamps, unique IDs, and time encodings.

## Source Code Example

Below is a Rust program demonstrating the usage of various functions from the `e9571_time_number` module.

```rust
use e9571_time_number::e9571_time_number::*;

fn main() {
    // Example 1: create_format_time
    println!("=== create_format_time Example ===");
    let formats = [
        "time", "msec", "micro", "nano", "unix", "unix_micro", 
        "unix_msec", "unix_nano", "time_str", "msec_str", 
        "micro_str", "nano_str", "dir", "invalid"
    ];
    for format in formats {
        println!("Format {}: {}", format, create_format_time(format));
    }

    // Example 2: unix_number
    println!("\n=== unix_number Example ===");
    let time_str = "2025-08-09 08:00:00";
    println!("Time string {} converted to Unix timestamp: {}", time_str, unix_number(time_str));

    // Example 3: unix_time
    println!("\n=== unix_time Example ===");
    let unix_str = "1723161600"; // Unix timestamp for 2025-08-09 08:00:00
    println!("Unix timestamp {} converted to time: {}", unix_str, unix_time(unix_str));

    // Example 4: time_standard
    println!("\n=== time_standard Example ===");
    let unix_millis = "1723161600000"; // Millisecond timestamp
    println!("Millisecond timestamp {} converted to Unix: {}", unix_millis, time_standard(unix_millis, "unix"));
    println!("Millisecond timestamp {} converted to time: {}", unix_millis, time_standard(unix_millis, "time"));

    // Example 5: group_id_sec
    println!("\n=== group_id_sec Example ===");
    println!("Unsigned Group ID: {}", group_id_sec(""));
    println!("Signed Group ID (BTC): {}", group_id_sec("BTC"));

    // Example 6: create_time_id
    println!("\n=== create_time_id Example ===");
    let time_str = "2025-08-09 08:00:00";
    let (time_sign, date_day, date_hour, date_minute) = create_time_id(time_str);
    println!("Time string {} parsed result:", time_str);
    println!(" time_sign: {}", time_sign);
    println!(" date_day: {}", date_day);
    println!(" date_hour: {}", date_hour);
    println!(" date_minute: {}", date_minute);

    // Example 7: data_sign_decode
    println!("\n=== data_sign_decode Example ===");
    let compact_time = "20250809080000";
    println!("Compact time {} decoded to: {}", compact_time, data_sign_decode(compact_time));

    // Example 8: data_sign_encode
    println!("\n=== data_sign_encode Example ===");
    let input_times = [
        "2025-08-09 08:00:00",
        "2025/08/09 08:00:00",
        "2025.08.09 08:00:00",
        "20250809 08:00:00",
        "2025-08-09T08:00:00",
    ];
    for input in input_times {
        println!("Time {} encoded to: {}", input, data_sign_encode(input));
    }

    // Example 9: get_time_str
    println!("\n=== get_time_str Example ===");
    let time_str = "2025-08-09 08:00:00";
    let components = ["y", "m", "d", "h", "i", "s", "invalid"];
    for component in components {
        println!("Component {} of time {}: {}", component, time_str, get_time_str(time_str, component));
    }

    // Example 10: random
    println!("\n=== random Example ===");
    println!("Random number (10000-99999): {}", random(10000, 99999));

    // Example 11: res_id
    println!("\n=== res_id Example ===");
    println!("High-precision sequential ID (type=TEST): {}", res_id("TEST"));

    // Example 12: generate_random_number
    println!("\n=== generate_random_number Example ===");
    match generate_random_number(1, 100) {
        Ok(num) => println!("Generated random number [1, 100]: {}", num),
        Err(e) => println!("Error generating random number: {}", e),
    }
}
```

## Explanation of Functions

The `e9571_time_number` module provides utility functions for handling time and number-related operations in a casino context, such as generating timestamps, encoding time formats, and creating unique IDs.

1. **`create_format_time`**: Generates a timestamp in various formats (e.g., time, millisecond, Unix timestamp).
2. **`unix_number`**: Converts a time string to a Unix timestamp.
3. **`unix_time`**: Converts a Unix timestamp to a readable time string.
4. **`time_standard`**: Converts a millisecond timestamp to different formats (e.g., Unix or time string).
5. **`group_id_sec`**: Generates a group ID, optionally signed with a currency (e.g., BTC).
6. **`create_time_id`**: Parses a time string into components (sign, day, hour, minute).
7. **`data_sign_decode`**: Decodes a compact time string into a readable format.
8. **`data_sign_encode`**: Encodes a time string into a compact format, supporting multiple input formats.
9. **`get_time_str`**: Extracts specific components (year, month, day, etc.) from a time string.
10. **`random`**: Generate a random number within a specified range using seconds.
11. **`res_id`**: Generates a high-precision sequential ID based on a type identifier.
12. **`generate_random_number`**: Generate a random number within a specified range using milliseconds.

## Casino Scenario Usage

These functions are useful in casino applications for:
- Tracking betting timestamps (`create_format_time`, `unix_number`, `unix_time`).
- Generating unique identifiers for bets or sessions (`group_id_sec`, `res_id`).
- Encoding/decoding time data for records or displays (`data_sign_encode`, `data_sign_decode`).
- Extracting time components for analytics or scheduling (`get_time_str`, `create_time_id`).
- Generating random numbers for game mechanics or IDs (`random`,`generate_random_number`).

## Notes
- The example assumes the `e9571_time_number` module is available and correctly implemented.
- The output of each function depends on the system time and specific implementation details.
- For production use, add error handling and input validation to ensure robustness.
