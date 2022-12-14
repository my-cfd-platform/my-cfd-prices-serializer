use chrono::{Datelike, Timelike};
use rust_extensions::date_time::DateTimeAsMicroseconds;

const OUR_MARKER: u8 = 'O' as u8;
const SOURCE_MARKER: u8 = 'S' as u8;

#[derive(Debug, Clone)]
pub enum BidAskDateTime {
    Source(DateTimeAsMicroseconds),
    Our(DateTimeAsMicroseconds),
}

impl BidAskDateTime {
    #[cfg(test)]
    pub fn unwrap_as_our_date(&self) -> &DateTimeAsMicroseconds {
        match self {
            BidAskDateTime::Our(data) => data,
            _ => panic!("BidAsk::unwrap_as_our_date: not Our Date"),
        }
    }
    #[cfg(test)]
    pub fn unwrap_as_source_date(&self) -> &DateTimeAsMicroseconds {
        match self {
            BidAskDateTime::Source(data) => data,
            _ => panic!("BidAsk::unwrap_as_source_date: not Source Date"),
        }
    }

    pub fn serialize(&self, dest: &mut Vec<u8>) {
        match &self {
            BidAskDateTime::Source(date_time) => {
                dest.push(SOURCE_MARKER as u8);
                date_time_to_string(dest, date_time);
            }
            BidAskDateTime::Our(date_time) => {
                dest.push(OUR_MARKER as u8);
                date_time_to_string(dest, date_time);
            }
        };
    }

    pub fn parse(src: &str) -> Self {
        let source_time = src.as_bytes()[0] != OUR_MARKER;
        let date_time = parse_date_time(&src[1..]);

        if source_time {
            BidAskDateTime::Source(date_time)
        } else {
            BidAskDateTime::Our(date_time)
        }
    }
}
fn parse_date_time(line: &str) -> DateTimeAsMicroseconds {
    let year: i32 = line[0..4].parse().unwrap();
    let month: u32 = line[4..6].parse().unwrap();
    let day: u32 = line[6..8].parse().unwrap();
    let hour: u32 = line[8..10].parse().unwrap();
    let min: u32 = line[10..12].parse().unwrap();
    let sec: u32 = line[12..14].parse().unwrap();

    println!()

    let micros_str = &line[14..];
    let mut micro: i64 = micros_str.parse().unwrap();

    match micros_str.len() {
        1 => {
            micro *= 100_000;
        }
        2 => {
            micro *= 10_000;
        }
        3 => {
            micro *= 1_000;
        }
        4 => {
            micro *= 100;
        }
        5 => {
            micro *= 10;
        }
        _ => {}
    }

    DateTimeAsMicroseconds::create(year, month, day, hour, min, sec, micro)
}

fn date_time_to_string(result: &mut Vec<u8>, dt: &DateTimeAsMicroseconds) {
    let dt = dt.to_chrono_utc();

    result.extend_from_slice(dt.year().to_string().as_bytes());

    push_with_leading_zero(result, dt.month() as u8);
    push_with_leading_zero(result, dt.day() as u8);
    push_with_leading_zero(result, dt.hour() as u8);
    push_with_leading_zero(result, dt.minute() as u8);
    push_with_leading_zero(result, dt.second() as u8);
    result.push('.' as u8);

    let mut ms_as_string = dt.nanosecond().to_string();

    let ms_as_slice = if ms_as_string.len() < 6 {
        while ms_as_string.len() < 3 {
            ms_as_string.push('0');
        }

        &ms_as_string
    } else {
        &ms_as_string[..6]
    };

    result.extend_from_slice(ms_as_slice.as_bytes());
}

fn push_with_leading_zero(result: &mut Vec<u8>, value: u8) {
    if value < 10 {
        result.push('0' as u8);
        let value = '0' as u8 + value;
        result.push(value);
    } else {
        result.extend_from_slice(value.to_string().as_bytes());
    }
}
