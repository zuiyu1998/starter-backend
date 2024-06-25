use chrono::{Local, NaiveDateTime};

pub fn get_now() -> NaiveDateTime {
    Local::now().naive_utc()
}
