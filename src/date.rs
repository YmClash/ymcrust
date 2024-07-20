use chrono::{DateTime, Utc};
use chrono::TimeZone;
use chrono::LocalResult;


// Dateutils module

//001
pub fn get_date() -> DateTime<Utc> {
    let now: DateTime<Utc> = Utc::now();
    return now;
}

//002
pub fn get_date_str() -> String {
    let now: DateTime<Utc> = Utc::now();
    return now.to_string();
}

//003
pub fn date_to_timestamps(date: DateTime<Utc>)  -> i64 {
    let timestamp = date.timestamp();
    return timestamp;
}

//004
pub fn timestamps_to_date(timestamps: i64) -> LocalResult<DateTime<Utc>>{
    let date = Utc.timestamp_opt(timestamps,0);
    return date;
}



