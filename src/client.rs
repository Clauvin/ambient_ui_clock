use ambient_api::prelude::*;
use std::f32::consts::PI;
use core::time::Duration;
use chrono::prelude::*;


const CLOCK_RAY: f32 = 250.;
const CLOCK_X_POSITION: f32 = 375.;
const CLOCK_Y_POSITION: f32 = 300.;
const CLOCK_X_CENTER: f32 = CLOCK_X_POSITION;
const CLOCK_Y_CENTER: f32 = CLOCK_Y_POSITION;

const HOUR_RAY: f32 = CLOCK_RAY/4.;
const MINUTE_RAY: f32 = CLOCK_RAY/3.;
const SECOND_RAY: f32 = CLOCK_RAY/2.;

const CLOCK_BORDER_COLOR: Vec4 = vec4(0.2, 0.1, 0.6, 1.);
const HOUR_COLOR: Vec4 = CLOCK_BORDER_COLOR;
const MINUTE_COLOR: Vec4 = vec4(0.2, 0.6, 0.1, 1.);
const SECOND_COLOR: Vec4 = vec4(0.6, 0.1, 0.2, 1.);

fn draw_circle(x_position: f32, y_position: f32, ray: f32, circle_border_color: Vec4) -> Element{
    Rectangle.el()
        .with(width(), ray*2.)
        .with(height(), ray*2.)
        .with(translation(), vec3(x_position-ray, y_position-ray, 0.01))
        .with(border_color(), circle_border_color)
        .with(border_thickness(), 4.)
        .with(border_radius(), vec4(ray,ray,ray,ray))
}

fn draw_hand(from_x: f32, from_y:f32, to_x: f32, to_y: f32, hand_color: Vec4) -> Element {
    Line.el()
    .with(line_from(), vec3(from_x, from_y, 0.0))
    .with(line_to(), vec3(to_x, to_y, 0.0))
    .with(line_width(), 4.)
    .with(background_color(), hand_color)
}

fn draw_static_hour_hand(from_x: f32, from_y:f32, to_x: f32, to_y: f32) -> Element {
    draw_hand(from_x, from_y, to_x, to_y, HOUR_COLOR)
}

fn draw_static_minute_hand(from_x: f32, from_y:f32, to_x: f32, to_y: f32) -> Element {
    draw_hand(from_x, from_y, to_x, to_y, MINUTE_COLOR)
}

fn draw_static_second_hand(from_x: f32, from_y:f32, to_x: f32, to_y: f32) -> Element {
    draw_hand(from_x, from_y, to_x, to_y, SECOND_COLOR)
}

fn get_current_hour(){}

#[element_component]
fn App(_hooks: &mut Hooks) -> Element {
    let size_info = _hooks.use_query(window_logical_size());

    let (now, set_now) = _hooks.use_state(time());

    let (hour_x, set_hour_x) = _hooks.use_state(CLOCK_X_CENTER);
    let (hour_y, set_hour_y) = _hooks.use_state(CLOCK_Y_CENTER - HOUR_RAY);
    let (minute_x, set_minute_x) = _hooks.use_state(CLOCK_X_CENTER);
    let (minute_y, set_minute_y) = _hooks.use_state(CLOCK_Y_CENTER - MINUTE_RAY);
    let (second_x, set_second_x) = _hooks.use_state(CLOCK_X_CENTER);
    let (second_y, set_second_y) = _hooks.use_state(CLOCK_Y_CENTER - SECOND_RAY);

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
            set_hour_x(CLOCK_X_CENTER + HOUR_RAY*(hour_phase.sin())+0.1);
            set_hour_y(CLOCK_Y_CENTER - HOUR_RAY*(hour_phase.cos())-0.1);

            set_minute_x(CLOCK_X_CENTER + MINUTE_RAY*(minute_phase.sin())+0.1);
            set_minute_y(CLOCK_Y_CENTER - MINUTE_RAY*(minute_phase.cos())-0.1);

            set_second_x(CLOCK_X_CENTER + SECOND_RAY*(second_phase.sin())+0.1);
            set_second_y(CLOCK_Y_CENTER - SECOND_RAY*(second_phase.cos())-0.1);
        }
    });

    Group::el([
        draw_circle(CLOCK_X_POSITION, CLOCK_Y_POSITION, CLOCK_RAY, CLOCK_BORDER_COLOR),
        draw_static_hour_hand(CLOCK_X_CENTER, CLOCK_Y_CENTER, hour_x, hour_y),
        draw_static_minute_hand(CLOCK_X_CENTER, CLOCK_Y_CENTER, minute_x, minute_y),
        draw_static_second_hand(CLOCK_X_CENTER, CLOCK_Y_CENTER, second_x, second_y),
    ])
}


#[main]
pub fn main() {
    color_tests();
	start();
}

fn start(){
	App.el().spawn_interactive();
}

//#[test]
fn color_test(color: Vec4) {
	let color_lower_limit = 0.;
	let color_higher_limit = 1.;
	more_asserts::debug_assert_ge!(color.x, color_lower_limit, "X value of color is smaller than {}", color_lower_limit);
	more_asserts::debug_assert_ge!(color.y, color_lower_limit, "Y value of color is smaller than {}", color_lower_limit);
	more_asserts::debug_assert_ge!(color.z, color_lower_limit, "Z value of color is smaller than {}", color_lower_limit);
	more_asserts::debug_assert_ge!(color.w, color_lower_limit, "W value of color is smaller than {}", color_lower_limit);
	more_asserts::debug_assert_le!(color.x, color_higher_limit, "X value of color is bigger than {}", color_higher_limit);
	more_asserts::debug_assert_le!(color.y, color_higher_limit, "Y value of color is bigger than {}", color_higher_limit);
	more_asserts::debug_assert_le!(color.z, color_higher_limit, "Z value of color is bigger than {}", color_higher_limit);
	more_asserts::debug_assert_le!(color.w, color_higher_limit, "W value of color is bigger than {}", color_higher_limit);
}

fn clock_border_color_test(){
	color_test(CLOCK_BORDER_COLOR);
}

fn hour_color_test(){
	color_test(HOUR_COLOR);
}

fn minute_color_test(){
	color_test(MINUTE_COLOR);
}

fn second_color_test(){
	color_test(SECOND_COLOR);
}

//the etc_color variables are Vec4 used for the clock colors
fn color_tests(){
	clock_border_color_test();
	hour_color_test();
	minute_color_test();
	second_color_test();
}