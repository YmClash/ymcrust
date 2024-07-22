use chrono::{DateTime, NaiveDate, Utc};
use chrono::TimeZone;



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
pub fn timestamps_to_date(timestamps: i64) -> String {
    let date = Utc.timestamp_opt(timestamps,0).unwrap().to_rfc2822();
    return date;
}

//005
pub fn is_valid_date(date_str: &str) -> bool {
    let format = "%d-%m-%Y" ;
//  let format = "%Y-%m-%d %H:%M:%S";
    match NaiveDate::parse_from_str(date_str,format){
        Ok(_) => true,
        Err(_) => false,
    }

}



