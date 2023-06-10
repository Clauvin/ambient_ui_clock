use ambient_api::prelude::*;
use std::f32::consts::PI;
use core::time::Duration;

const clock_ray: f32 = 250.;
const clock_x_position: f32 = 375.;
const clock_y_position: f32 = 300.;
const clock_x_center: f32 = clock_x_position;
const clock_y_center: f32 = clock_y_position;

const hour_ray: f32 = clock_ray/4.;
const minute_ray: f32 = clock_ray/3.;
const second_ray: f32 = clock_ray/2.;

const clock_border_color: Vec4 = vec4(0.2, 0.1, 0.6, 1.);
const hour_color: Vec4 = clock_border_color;
const minute_color: Vec4 = vec4(0.2, 0.6, 0.1, 1.);
const second_color: Vec4 = vec4(0.6, 0.1, 0.2, 1.);

fn DrawCircle(x_position: f32, y_position: f32, ray: f32, circle_border_color: Vec4) -> Element{
    Rectangle.el()
        .with(width(), ray*2.)
        .with(height(), ray*2.)
        .with(translation(), vec3(x_position-ray, y_position-ray, 0.01))
        .with(border_color(), circle_border_color)
        .with(border_thickness(), 4.)
        .with(border_radius(), vec4(ray,ray,ray,ray))
}

fn DrawHand(from_x: f32, from_y:f32, to_x: f32, to_y: f32, hand_color: Vec4) -> Element {
    Line.el()
    .with(line_from(), vec3(from_x, from_y, 0.0))
    .with(line_to(), vec3(to_x, to_y, 0.0))
    .with(line_width(), 4.)
    .with(background_color(), hand_color)
}

fn DrawStaticHourHand(from_x: f32, from_y:f32, to_x: f32, to_y: f32) -> Element {
    DrawHand(from_x, from_y, to_x, to_y, hour_color)
}

fn DrawStaticMinuteHand(from_x: f32, from_y:f32, to_x: f32, to_y: f32) -> Element {
    DrawHand(from_x, from_y, to_x, to_y, minute_color)
}

fn DrawStaticSecondHand(from_x: f32, from_y:f32, to_x: f32, to_y: f32) -> Element {
    DrawHand(from_x, from_y, to_x, to_y, second_color)
}

#[element_component]
fn App(_hooks: &mut Hooks) -> Element {
    let size_info = _hooks.use_query(window_logical_size());

    let (now, set_now) = _hooks.use_state(time());

    let (hour_x, set_hour_x) = _hooks.use_state(clock_x_center);
    let (hour_y, set_hour_y) = _hooks.use_state(clock_y_center - hour_ray);
    let (minute_x, set_minute_x) = _hooks.use_state(clock_x_center);
    let (minute_y, set_minute_y) = _hooks.use_state(clock_y_center - minute_ray);
    let (second_x, set_second_x) = _hooks.use_state(clock_x_center);
    let (second_y, set_second_y) = _hooks.use_state(clock_y_center - second_ray);

    let (hour_phase, set_hour_phase) = _hooks.use_state(PI/(1800.*24.));
    let (minute_phase, set_minute_phase) = _hooks.use_state(PI/1800.);
    let (second_phase, set_second_phase) = _hooks.use_state(PI/30.);

    _hooks.use_frame(move |world|{
        let latest = time();
        if latest - now > Duration::from_secs_f32(1.0).as_secs_f32() {
            set_now(latest);
            set_hour_phase({
                if hour_phase + PI/(1800.*24.) > PI*2.0 {
                    hour_phase + PI/(1800.*24.) - PI*2.0
                } else {
                    hour_phase + PI/(1800.*24.)
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
        DrawCircle(clock_x_position, clock_y_position, clock_ray, clock_border_color),
        DrawStaticHourHand(clock_x_center, clock_y_center, hour_x, hour_y),
        DrawStaticMinuteHand(clock_x_center, clock_y_center, minute_x, minute_y),
        DrawStaticSecondHand(clock_x_center, clock_y_center, second_x, second_y),
    ])
}


#[main]
pub fn main() {
    App.el().spawn_interactive();
}