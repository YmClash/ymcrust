#[cfg(test)]
mod tests {
   // use ymcrust::date_to_timestamps;

    #[test]
    fn test_date() {
        use ymcrust::get_date;
        let result = get_date();
        assert!(result.timestamp() > 0)

    }

    #[test]
    fn test_get_date_str() {
        use ymcrust::{get_date,get_date_str,type_of};
        let result = get_date_str();
        let now = get_date().to_string();
        assert_eq!( type_of(result),type_of(now));
    }

    #[test]
    fn test_date_to_timestamps(){
        use ymcrust::{date_to_timestamps, get_date};
        let now = get_date();
        let timestamp = date_to_timestamps(now);
        assert_eq!(now.timestamp(),timestamp)
    }

    #[test]
    fn test_timestamps_to_date(){
        use ymcrust::{timestamps_to_date};
        let date_timestamp = timestamps_to_date(720144000);
        assert_eq!(date_timestamp,"Tue, 27 Oct 1992 00:00:00 +0000");
    }

    #[test]
    fn test_is_valid_date(){
        use ymcrust::{is_valid_date};
        let date = "27-10-1992";
        let date_2 = "2700-10-27";

        assert_eq!(is_valid_date(date),true);
        assert_eq!(is_valid_date(date_2),false);
    }


}