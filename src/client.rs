use ambient_api::prelude::*;
use std::f32::consts::PI;
use core::time::Duration;
use chrono::prelude::*;

mod drawing;
mod clock_time;
mod tests;

#[element_component]
fn App(_hooks: &mut Hooks) -> Element {
    let size_info = _hooks.use_query(window_logical_size());

    let (now, set_now) = _hooks.use_state(time());


    //Have to make set code for this so it won't repeat itself.
    let initial_date_and_time = clock_time::get_current_date_and_time();
    let initial_clock_hour = clock_time::get_current_hour12(initial_date_and_time) as f32;
    let initial_clock_minute = clock_time::get_current_minutes(initial_date_and_time) as f32;
    let initial_clock_second = clock_time::get_current_seconds(initial_date_and_time) as f32;

    let mut initial_ray = 0.;
    if size_info[0].1.x <= size_info[0].1.y {
        initial_ray = (size_info[0].1.x/2) as f32;
    } else {
        initial_ray = (size_info[0].1.y/2) as f32;
    }

    let (clock_ray, set_clock_ray) = _hooks.use_state(initial_ray);
    let (clock_x_position, set_clock_x_position) = _hooks.use_state(clock_ray);
    let (clock_y_position, set_clock_y_position) = _hooks.use_state(clock_ray);
    let (clock_x_center, set_clock_x_center) = _hooks.use_state(clock_x_position);
    let (clock_y_center, set_clock_y_center) = _hooks.use_state(clock_y_position);

    //same.
    let hour_ray: f32 = clock_ray/5.;
    let minute_ray: f32 = clock_ray/3.;
    let second_ray: f32 = clock_ray/2.;

    let (hour_x, set_hour_x) = _hooks.use_state(clock_x_center);
    let (hour_y, set_hour_y) = _hooks.use_state(clock_y_center - hour_ray);
    let (minute_x, set_minute_x) = _hooks.use_state(clock_x_center);
    let (minute_y, set_minute_y) = _hooks.use_state(clock_y_center - minute_ray);
    let (second_x, set_second_x) = _hooks.use_state(clock_x_center);
    let (second_y, set_second_y) = _hooks.use_state(clock_y_center - second_ray);

    let (hour_phase, set_hour_phase) = _hooks.use_state(
        clock_time::get_initial_hour_phase(initial_clock_hour, initial_clock_minute, initial_clock_second));
    let (minute_phase, set_minute_phase) = _hooks.use_state(
        clock_time::get_initial_minute_phase(initial_clock_minute, initial_clock_second));
    let (second_phase, set_second_phase) = _hooks.use_state(
        clock_time::get_initial_second_phase(initial_clock_second));

    set_hour_x(clock_x_center + hour_ray*(hour_phase.sin())+0.1);
    set_hour_y(clock_y_center - hour_ray*(hour_phase.cos())-0.1);

    set_minute_x(clock_x_center + minute_ray*(minute_phase.sin())+0.1);
    set_minute_y(clock_y_center - minute_ray*(minute_phase.cos())-0.1);

    set_second_x(clock_x_center + second_ray*(second_phase.sin())+0.1);
    set_second_y(clock_y_center - second_ray*(second_phase.cos())-0.1);

    _hooks.use_frame(move |world|{
        let window_width = size_info[0].1.x;
        let window_height = size_info[0].1.y;

        let mut center = 0.;

        if window_width <= window_height{
            center = (window_width/2) as f32;
        } else {
            center = (window_height/2) as f32;
        }

        set_clock_ray(center);
        set_clock_x_center(center); set_clock_y_center(center);
        set_clock_x_position(center); set_clock_y_position(center);

        let latest = time();
        if latest - now > Duration::from_secs_f32(1.0).as_secs_f32() {
            set_now(latest);
            set_hour_phase({
                if hour_phase + PI/(1800.*12.) > PI*2.0 {
                    hour_phase + PI/(1800.*12.) - PI*2.0
                } else {
                    hour_phase + PI/(1800.*12.)
                }
            });
            set_minute_phase({
                if minute_phase + PI/1800.0 > PI*2.0 {
                    minute_phase + PI/1800.0 - PI*2.0
                } else {
                    minute_phase + PI/1800.0
                }
            });            
            set_second_phase({
                if second_phase + PI/30.0 > PI*2.0 {
                    second_phase + PI/30.0 - PI*2.0
                } else {
                    second_phase + PI/30.0
                }
            });

            // for some reason, second 45 without 0.1 won't show.
            // Maybe it happens with minute and hour 45 too, so I'm adding the same fix to those as well
            // That said, I need to check those later, properly
            set_hour_x(clock_x_center + hour_ray*(hour_phase.sin())+0.1);
            set_hour_y(clock_y_center - hour_ray*(hour_phase.cos())-0.1);

            set_minute_x(clock_x_center + minute_ray*(minute_phase.sin())+0.1);
            set_minute_y(clock_y_center - minute_ray*(minute_phase.cos())-0.1);

            set_second_x(clock_x_center + second_ray*(second_phase.sin())+0.1);
            set_second_y(clock_y_center - second_ray*(second_phase.cos())-0.1);
        }
    });

    Group::el([
        drawing::draw_circle(clock_x_position, clock_y_position, clock_ray, drawing::CLOCK_BORDER_COLOR),
        drawing::draw_static_hour_hand(clock_x_center, clock_y_center, hour_x, hour_y),
        drawing::draw_static_minute_hand(clock_x_center, clock_y_center, minute_x, minute_y),
        drawing::draw_static_second_hand(clock_x_center, clock_y_center, second_x, second_y),
    ])
}


#[main]
pub fn main() {
    println!("{:?}", Local::now());
    tests::color_tests();
	start();
}

fn start(){
	App.el().spawn_interactive();
}

