use ambient_api::prelude::*;
use std::f32::consts::PI;
use core::time::Duration;
use chrono::prelude::*;

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

fn get_current_date_and_time() -> DateTime<Local>{
    Local::now()
}

fn get_current_hour12(date_and_time: DateTime<Local>) -> u32 {
    date_and_time.hour12().1
}

fn get_current_minutes(date_and_time: DateTime<Local>) -> u32 {
    date_and_time.minute()
}

fn get_current_seconds(date_and_time: DateTime<Local>) -> u32 {
    date_and_time.second()
}



#[element_component]
fn App(_hooks: &mut Hooks) -> Element {

    let size_info = _hooks.use_query(window_logical_size());

    let (now, set_now) = _hooks.use_state(time());

    let initial_date_and_time = get_current_date_and_time();
    let initial_clock_hour = get_current_hour12(initial_date_and_time) as f32;
    let initial_clock_minute = get_current_minutes(initial_date_and_time) as f32;
    let initial_clock_second = get_current_seconds(initial_date_and_time) as f32;

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

    let hour_ray: f32 = clock_ray/4.;
    let minute_ray: f32 = clock_ray/3.;
    let second_ray: f32 = clock_ray/2.;

    let (hour_x, set_hour_x) = _hooks.use_state(clock_x_center);
    let (hour_y, set_hour_y) = _hooks.use_state(clock_y_center - hour_ray);
    let (minute_x, set_minute_x) = _hooks.use_state(clock_x_center);
    let (minute_y, set_minute_y) = _hooks.use_state(clock_y_center - minute_ray);
    let (second_x, set_second_x) = _hooks.use_state(clock_x_center);
    let (second_y, set_second_y) = _hooks.use_state(clock_y_center - second_ray);

    let (hour_phase, set_hour_phase) = _hooks.use_state(initial_clock_hour * PI/12.);
    let (minute_phase, set_minute_phase) = _hooks.use_state(initial_clock_minute * PI/30.);
    let (second_phase, set_second_phase) = _hooks.use_state(initial_clock_second * PI/30.);

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
        draw_circle(clock_x_position, clock_y_position, clock_ray, CLOCK_BORDER_COLOR),
        draw_static_hour_hand(clock_x_center, clock_y_center, hour_x, hour_y),
        draw_static_minute_hand(clock_x_center, clock_y_center, minute_x, minute_y),
        draw_static_second_hand(clock_x_center, clock_y_center, second_x, second_y),
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