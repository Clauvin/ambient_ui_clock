use chrono::prelude::*;
use std::f32::consts::PI;

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

pub fn get_initial_hour_phase(initial_clock_hour: f32, initial_clock_minute: f32, initial_clock_second: f32) -> f32 {
    initial_clock_hour * PI/6. + initial_clock_minute * PI/6. * 1./60. + initial_clock_second * PI/6. * 1./60. * 1./60.
}

pub fn get_initial_minute_phase(initial_clock_minute: f32, initial_clock_second: f32) -> f32 {
    initial_clock_minute * PI/30. + initial_clock_second * PI/30. * 1./60.
}

pub fn get_initial_second_phase(initial_clock_second: f32) -> f32 {
    initial_clock_second * PI/30.
}

pub fn hour_hand_update(hour_phase: f32) -> f32 {
    if hour_phase + PI/(1800.*12.) > PI*2.0 {
        hour_phase + PI/(1800.*12.) - PI*2.0
    } else {
        hour_phase + PI/(1800.*12.)
    }
}

pub fn minute_hand_update(minute_phase: f32) -> f32 {
    if minute_phase + PI/1800.0 > PI*2.0 {
        minute_phase + PI/1800.0 - PI*2.0
    } else {
        minute_phase + PI/1800.0
    }
}

pub fn second_hand_update(second_phase: f32) -> f32 {
    if second_phase + PI/30.0 > PI*2.0 {
        second_phase + PI/30.0 - PI*2.0
    } else {
        second_phase + PI/30.0
    }
}