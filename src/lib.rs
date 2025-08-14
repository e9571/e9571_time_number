use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Local, TimeZone};
use rand::{thread_rng, Rng};

pub mod e9571_time_number {
    use super::*;

    /// Go 语言时间专用格式化函数，返回常规时间、文件名时间或 Unix 时间
    pub fn create_format_time(type_str: &str) -> String {
        let now = SystemTime::now();
        let timestamp = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let datetime: DateTime<Local> = DateTime::from(now);

        let mut str_time = String::new();

        match type_str {
            "time" => str_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            "msec" => str_time = datetime.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
            "micro" => str_time = datetime.format("%Y-%m-%d %H:%M:%S%.6f").to_string(),
            "nano" => str_time = datetime.format("%Y-%m-%d %H:%M:%S%.9f").to_string(),
            "unix" => str_time = timestamp.to_string(),
            "unix_micro" => {
                let micros = now.duration_since(UNIX_EPOCH).unwrap().as_micros();
                str_time = (micros / 1000).to_string();
            }
            "unix_msec" => {
                let nanos = now.duration_since(UNIX_EPOCH).unwrap().as_nanos();
                str_time = (nanos / 1_000_000).to_string();
            }
            "unix_nano" => {
                let nanos = now.duration_since(UNIX_EPOCH).unwrap().as_nanos();
                str_time = nanos.to_string();
            }
            "time_str" => str_time = datetime.format("%Y%m%d%H%M%S").to_string(),
            "msec_str" => {
                str_time = datetime.format("%Y%m%d%H%M%S%.3f").to_string();
                str_time = str_time.replace(".", "");
            }
            "micro_str" => {
                str_time = datetime.format("%Y%m%d%H%M%S%.6f").to_string();
                str_time = str_time.replace(".", "");
            }
            "nano_str" => {
                str_time = datetime.format("%Y%m%d%H%M%S%.9f").to_string();
                str_time = str_time.replace(".", "");
            }
            "dir" => str_time = datetime.format("%Y%m%d").to_string(),
            _ => str_time = "help:time,msec,micro,nano,unix,unix_micro,unix_msec,unix_nano,time_str,msec_str,micro_str,nano_str".to_string(),
        }

        str_time
    }

    /// 将指定时间格式转换为 Unix 时间戳
    pub fn unix_number(time_str: &str) -> String {
        let time_str = time_str.replace("/", "-");
        let loc = chrono::offset::FixedOffset::east_opt(8 * 3600).unwrap(); // Asia/Shanghai
        match DateTime::parse_from_str(&time_str, "%Y-%m-%d %H:%M:%S") {
            Ok(dt) => dt.timestamp().to_string(),
            Err(_) => String::new(),
        }
    }

    /// 将 Unix 时间戳转换为指定时间格式
    pub fn unix_time(unix_str: &str) -> String {
        if unix_str.len() != 10 {
            return unix_str.to_string();
        }
        match unix_str.parse::<i64>() {
            Ok(int_value) => {
                let dt = Local.timestamp_opt(int_value, 0).unwrap();
                dt.format("%Y-%m-%d %H:%M:%S").to_string()
            }
            Err(_) => unix_str.to_string(),
        }
    }

    /// 获取币安标准时间，返回 Unix 或时间格式
    pub fn time_standard(unix: &str, type_str: &str) -> String {
        if unix.len() != 13 {
            return String::new();
        }
        let unix = &unix[0..unix.len() - 3];
        match type_str {
            "unix" => unix.to_string(),
            _ => unix_time(unix),
        }
    }

    /// 生成高精度秒级 Group ID
    pub fn group_id_sec(symbol: &str) -> String {
        let mut group_id = create_format_time("time");
        group_id = group_id.replace(" ", "").replace(":", "").replace("-", "");
        if !symbol.is_empty() {
            format!("{}_{}", symbol, group_id)
        } else {
            group_id
        }
    }

    /// 格式化指定时间，生成数据简写与时序标志位
    pub fn create_time_id(time_str: &str) -> (String, String, String, String) {
        let layout = "%Y-%m-%d %H:%M:%S";
        match Local.datetime_from_str(time_str, layout) {
            Ok(utc_time) => {
                let time_sign = utc_time.format("%Y%m%d%H%M%S").to_string();
                let date_day = utc_time.format("%Y%m%d").to_string();
                let date_hour = utc_time.format("%d%H").to_string();
                let date_minute = utc_time.format("%H%M").to_string();
                (time_sign, date_day, date_hour, date_minute)
            }
            Err(_) => (String::new(), String::new(), String::new(), String::new()),
        }
    }

    /// 将时间字符串从 "20060102150405" 格式转换为 "2006-01-02 15:04:05"
    pub fn data_sign_decode(input: &str) -> String {
        match Local.datetime_from_str(input, "%Y%m%d%H%M%S") {
            Ok(t) => t.format("%Y-%m-%d %H:%M:%S").to_string(),
            Err(_) => {
                println!("invalid time format, expected YYYYMMDDHHMMSS");
                String::new()
            }
        }
    }

    /// 将任意格式的时间字符串转换为 "20060102150405" 格式
    pub fn data_sign_encode(input: &str) -> String {
        let formats = vec![
            "%Y-%m-%d %H:%M:%S",
            "%Y/%m/%d %H:%M:%S",
            "%Y.%m.%d %H:%M:%S",
            "%Y%m%d %H:%M:%S",
            "%Y-%m-%dT%H:%M:%S",
            "%Y-%m-%d",
            "%Y-%m-%dT%H:%M:%S%.fZ",
            "%a, %d %b %Y %H:%M:%S GMT",
            "%d %b %y %H:%M GMT",
        ];

        for format in formats {
            if let Ok(parsed_time) = Local.datetime_from_str(input, format) {
                return parsed_time.format("%Y%m%d%H%M%S").to_string();
            }
        }

        println!("unable to parse time string with any known format");
        String::new()
    }

    /// 获取时间戳中的指定标志位（年、月、日、小时、分、秒）
    pub fn get_time_str(value: &str, type_str: &str) -> String {
        if value.len() != 19 {
            println!("Length_err {} {}", value.len(), value);
            return value.to_string();
        }

        let value = value.replace("/", "-");
        let (time_id, _, _, _) = create_time_id(&value);
        if time_id.len() != 14 {
            println!("Length_err {} {}", time_id.len(), time_id);
            return value.to_string();
        }

        match type_str.to_lowercase().as_str() {
            "y" => time_id[0..4].to_string(),
            "m" => time_id[4..6].to_string(),
            "d" => time_id[6..8].to_string(),
            "h" => time_id[8..10].to_string(),
            "i" => time_id[10..12].to_string(),
            "s" => time_id[12..14].to_string(),
            _ => value.to_string(),
        }
    }

    /// 生成 min 到 max 之间的随机整数
    pub fn random(min: i32, max: i32) -> i32 {
        thread_rng().gen_range(min..max)
    }

    /// 生成高精度时序 ID
    pub fn res_id(type_str: &str) -> String {
        let myrand = random(10000, 99999);
        format!("{}{}{}", create_format_time("msec_str"), myrand, type_str)
    }
}