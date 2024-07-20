#[cfg(test)]
mod tests {

    use chrono::Utc;
    use ymcrust::get_date;

    #[test]
    fn test_date() {
        use ymcrust::get_date;
        let result = get_date();
        assert!(result.timestamp() > 0)

    }

    #[test]
    fn test_get_date_str() {
        use ymcrust::{get_date_str,type_of};
        let result = get_date_str();
        let now = get_date().to_string();
        assert_eq!( type_of(result),type_of(now));
    }
    //
    // #[test]
    // fn test_date_to_timestamps(){
    //     use ymcrust::{date_to_timestamps, get_date};
    //     let now = gete_date();
    //     let timestamp = date_to_timestamps(now);
    //     assert_eq!(now.timestamp(),timestamp)
    // }
    //
    // #[test]
    // fn test_timestamps_to_date(){
    //     use ymcrust::{timestamps_to_date, get_date, date_to_timestamps};
    //     let now = get_date();
    //     let timestamp = date_to_timestamps(now);
    //     let date = timestamps_to_date(timestamp);
    //     assert_eq!(now, date);
    // }
}