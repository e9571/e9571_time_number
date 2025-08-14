# e9571_time_number Call example

use e9571_time_number::e9571_time_number::*;

fn main() {
    // 示例 1: create_format_time
    println!("=== create_format_time 示例 ===");  
    let formats = [
        "time", "msec", "micro", "nano", "unix", "unix_micro",
        "unix_msec", "unix_nano", "time_str", "msec_str",
        "micro_str", "nano_str", "dir", "invalid"
    ];  
    for format in formats {
        println!("格式 {}: {}", format, create_format_time(format));
    }

    // 示例 2: unix_number
    println!("\n=== unix_number 示例 ===");
    let time_str = "2025-08-09 08:00:00";
    println!("时间字符串 {} 转换为 Unix 时间戳: {}", time_str, unix_number(time_str));

    // 示例 3: unix_time
    println!("\n=== unix_time 示例 ===");
    let unix_str = "1723161600"; // 2025-08-09 08:00:00 的 Unix 时间戳
    println!("Unix 时间戳 {} 转换为时间: {}", unix_str, unix_time(unix_str));

    // 示例 4: time_standard
    println!("\n=== time_standard 示例 ===");
    let unix_millis = "1723161600000"; // 毫秒级时间戳
    println!("毫秒时间戳 {} 转换为 Unix: {}", unix_millis, time_standard(unix_millis, "unix"));
    println!("毫秒时间戳 {} 转换为时间: {}", unix_millis, time_standard(unix_millis, "time"));

    // 示例 5: group_id_sec
    println!("\n=== group_id_sec 示例 ===");
    println!("无符号 Group ID: {}", group_id_sec(""));
    println!("带符号 Group ID (BTC): {}", group_id_sec("BTC"));

    // 示例 6: create_time_id
    println!("\n=== create_time_id 示例 ===");
    let time_str = "2025-08-09 08:00:00";
    let (time_sign, date_day, date_hour, date_minute) = create_time_id(time_str);
    println!("时间字符串 {} 解析结果:", time_str);
    println!("  time_sign: {}", time_sign);
    println!("  date_day: {}", date_day);
    println!("  date_hour: {}", date_hour);
    println!("  date_minute: {}", date_minute);

    // 示例 7: data_sign_decode
    println!("\n=== data_sign_decode 示例 ===");
    let compact_time = "20250809080000";
    println!("紧凑时间 {} 解码为: {}", compact_time, data_sign_decode(compact_time));

    // 示例 8: data_sign_encode
    println!("\n=== data_sign_encode 示例 ===");
    let input_times = [
        "2025-08-09 08:00:00",
        "2025/08/09 08:00:00",
        "2025.08.09 08:00:00",
        "20250809 08:00:00",
        "2025-08-09T08:00:00",
    ];
    for input in input_times {
        println!("时间 {} 编码为: {}", input, data_sign_encode(input));
    }

    // 示例 9: get_time_str
    println!("\n=== get_time_str 示例 ===");
    let time_str = "2025-08-09 08:00:00";
    let components = ["y", "m", "d", "h", "i", "s", "invalid"];
    for component in components {
        println!("时间 {} 的 {} 部分: {}", time_str, component, get_time_str(time_str, component));
    }

    // 示例 10: random
    println!("\n=== random 示例 ===");
    println!("随机数 (10000-99999): {}", random(10000, 99999));

    // 示例 11: res_id
    println!("\n=== res_id 示例 ===");
    println!("高精度时序 ID (type=TEST): {}", res_id("TEST"));
}
