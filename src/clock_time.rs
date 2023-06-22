use chrono::prelude::*;

pub fn get_current_date_and_time() -> DateTime<Local>{
    Local::now()
}

pub fn get_current_hour12(date_and_time: DateTime<Local>) -> u32 {
    date_and_time.hour12().1
}

pub fn get_current_minutes(date_and_time: DateTime<Local>) -> u32 {
    date_and_time.minute()
}

pub fn get_current_seconds(date_and_time: DateTime<Local>) -> u32 {
    date_and_time.second()
}

//This code should work, once the ambient bug of Local::now is properly fixed.
//Once it is, I will update the code to properly consider GMT.
pub fn get_current_time_zone() -> i32{
    Local::now().offset().local_minus_utc()
}