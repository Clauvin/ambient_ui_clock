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


//For some reason, it can't be drawn under the circle? If it is, then
//  it does not show up. If it is not, it always stays above the circle.
fn CreateWhiteBackground(_hooks: &mut Hooks) -> Element {
    let size_info = _hooks.use_query(window_logical_size());
    let x = size_info[0].1.x as f32;
    let y = size_info[0].1.y as f32;
    Rectangle.el()
        .with(width(), x)
        .with(height(), y)
        .with(background_color(), vec4(1., 1., 1., 1.))
        .with(translation(), vec3(0., 0., -1.01))
}

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

    let (minute_x, set_minute_x) = _hooks.use_state(size_info[0].1.x as f32 / 2.);
    let (minute_y, set_minute_y) = _hooks.use_state(size_info[0].1.y as f32 / 2.);
    let (second_x, set_second_x) = _hooks.use_state(size_info[0].1.x as f32 / 2.);
    let (second_y, set_second_y) = _hooks.use_state(size_info[0].1.y as f32 / 2.);

    let (phase, set_phase) = _hooks.use_state(PI/30.);

    _hooks.use_frame(move |world|{
        let latest = time();
        if latest - now > Duration::from_secs_f32(1.0).as_secs_f32() {
            set_now(latest);
            set_phase({
                if phase + PI/30.0 > PI*2.0 {
                    phase + PI/30.0 - PI*2.0
                } else {
                    phase + PI/30.0
                }
            });
            
            set_minute_x(clock_x_center + minute_ray*(phase.sin()));
            set_minute_y(clock_y_center - minute_ray*(phase.cos()));

            set_second_x(clock_x_center + second_ray*(phase.sin()));
            set_second_y(clock_y_center - second_ray*(phase.cos()));
            println!("hour_x: {}, hour_y: {}", second_x, second_y);
        }
    });

    Group::el([
        CreateWhiteBackground(_hooks),
        DrawCircle(clock_x_position, clock_y_position, clock_ray, clock_border_color),
        DrawStaticMinuteHand(clock_x_center, clock_y_center, minute_x, minute_y),
        DrawStaticSecondHand(clock_x_center, clock_y_center, second_x, second_y)
    ])
}


#[main]
pub fn main() {
    App.el().spawn_interactive();
}

