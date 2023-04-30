use chrono::{DateTime, Datelike, Days, Local};

pub fn get_this_monday() -> DateTime<Local> {
    let days_from_monday = Local::now().date_naive().weekday().num_days_from_monday();
    Local::now().checked_sub_days(Days::new(days_from_monday as u64)).unwrap()
}

pub fn get_this_friday() -> DateTime<Local> {
    let days_from_monday: u64 = Local::now().date_naive().weekday().num_days_from_monday().into();
    if days_from_monday <= 4 {
        Local::now().checked_add_days(Days::new(4 - days_from_monday)).unwrap()
    } else {
        Local::now().checked_sub_days(Days::new(days_from_monday - 4)).unwrap()
    }
}

pub fn format_for_untis(time: DateTime<Local>) -> String {
    format!("{}", time.format("%Y%m%d"))
}
