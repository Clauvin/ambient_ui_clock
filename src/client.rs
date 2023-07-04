use ambient_api::prelude::*;
use tests::clock_size_test;

use core::time::Duration;
use ambient_api::window;

mod drawing;
mod clock_time;
mod tests;

static mut do_clock_size_test: bool = false;

#[element_component]
fn App(_hooks: &mut Hooks) -> Element {
    let size_info = _hooks.use_query(window_logical_size());
    let window_width_for_ui = size_info[0].1.x as f32;

    let (its_now, set_its_now) = _hooks.use_state(time());

    let date_and_time = clock_time::get_current_date_and_time();
    let clock_hour = clock_time::get_current_hour12(date_and_time) as f32;
    let clock_minute = clock_time::get_current_minutes(date_and_time) as f32;
    let clock_second = clock_time::get_current_seconds(date_and_time) as f32;

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
        clock_time::get_hour_phase(clock_hour, clock_minute, clock_second));
    let (minute_phase, set_minute_phase) = _hooks.use_state(
        clock_time::get_minute_phase(clock_minute, clock_second));
    let (second_phase, set_second_phase) = _hooks.use_state(
        clock_time::get_second_phase(clock_second));

    set_hour_x(clock_x_center + hour_ray*(hour_phase.sin())+0.1);
    set_hour_y(clock_y_center - hour_ray*(hour_phase.cos())-0.1);

    set_minute_x(clock_x_center + minute_ray*(minute_phase.sin())+0.1);
    set_minute_y(clock_y_center - minute_ray*(minute_phase.cos())-0.1);

    set_second_x(clock_x_center + second_ray*(second_phase.sin())+0.1);
    set_second_y(clock_y_center - second_ray*(second_phase.cos())-0.1);

    //Yeah, for sure there's a much better way to do this. I'm going to work on it once ambient gets a good test system
    //Also, why this test breaks the drawing of UI?
    unsafe {
        if do_clock_size_test {
            clock_size_test(size_info[0].1.x, size_info[0].1.y);
            window::set_fullscreen(true);
            let size_info = _hooks.use_query(window_logical_size());
            clock_size_test(size_info[0].1.x, size_info[0].1.y);
            window::set_fullscreen(false);
            do_clock_size_test = false;
        }
    }

    _hooks.use_frame(move |world|{
        let window_width = size_info[0].1.x;
        let window_height = size_info[0].1.y;

        let center = drawing::define_clock_center(window_width, window_height);

        set_clock_ray(center);
        set_clock_x_center(center); set_clock_y_center(center);
        set_clock_x_position(center); set_clock_y_position(center);

        let latest = time();
        if latest - its_now > Duration::from_secs_f32(1.0).as_secs_f32() {
            set_its_now(latest);
            let date_and_time = clock_time::get_current_date_and_time();
            let clock_hour = clock_time::get_current_hour12(date_and_time) as f32;
            let clock_minute = clock_time::get_current_minutes(date_and_time) as f32;
            let clock_second = clock_time::get_current_seconds(date_and_time) as f32;

            //Originally, the code here would change the time based on the fact that one second had passed, 
            //  adding that one second to the movement of the hands.
            //  The problem is that there isn't enough precision on f32 to do that without adding gradual
            //  imprecisions to the clock.
            //  The current method is less imprecise and it does not leads to cumulative imprecision.
            set_hour_phase({
                clock_time::get_hour_phase(clock_hour, clock_minute, clock_second)
            });
            set_minute_phase({
                clock_time::get_minute_phase(clock_minute, clock_second)
            });            
            set_second_phase({
                clock_time::get_second_phase(clock_second)
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

    let (border_color_toggle, set_border_color_toggle) = _hooks.use_state(false);

    let (border_color_red, set_border_color_red) =
     _hooks.use_state(drawing::CLOCK_BORDER_COLOR.x);
    let (border_color_green, set_border_color_green) =
     _hooks.use_state(drawing::CLOCK_BORDER_COLOR.y);
    let (border_color_blue, set_border_color_blue) =
     _hooks.use_state(drawing::CLOCK_BORDER_COLOR.z);
    let (border_color_alpha, set_border_color_alpha) =
     _hooks.use_state(drawing::CLOCK_BORDER_COLOR.w);

    let (hour_hand_color_toggle, set_hour_hand_color_toggle) = _hooks.use_state(false);

    let (hour_hand_color_red, set_hour_hand_color_red) =
     _hooks.use_state(drawing::HOUR_COLOR.x);
    let (hour_hand_color_green, set_hour_hand_color_green) =
     _hooks.use_state(drawing::HOUR_COLOR.y);
    let (hour_hand_color_blue, set_hour_hand_color_blue) =
     _hooks.use_state(drawing::HOUR_COLOR.z);
    let (hour_hand_color_alpha, set_hour_hand_color_alpha) =
     _hooks.use_state(drawing::HOUR_COLOR.w);

    let row = |name, editor| FlowRow::el(vec![Text::el(name).with(min_width(), 110.), editor]);
    Group::el([
        FocusRoot::el([FlowColumn::el([
            Button::new("Border color config", move |_| {set_border_color_toggle(!border_color_toggle)})
                .hotkey(VirtualKeyCode::Q)
                .el(),
            if border_color_toggle {
                FlowColumn::el([
                row(
                    "Border Red Value",
                    F32Input {
                        value: border_color_red,
                        on_change: set_border_color_red,
                    }
                    .el(),
                ),
                row(
                    "Border Green Value",
                    F32Input {
                        value: border_color_green, 
                        on_change: set_border_color_green,
                    }
                    .el(),
                ),
                row(
                    "Border Blue Value",
                    F32Input {
                        value: border_color_blue, 
                        on_change: set_border_color_blue,
                    }
                    .el(),
                ),
                row(
                    "Border Alpha Value",
                    F32Input {
                        value: border_color_alpha,
                        on_change: set_border_color_alpha,
                    }
                    .el(),
                )])
            } else {Element::new()},

            Button::new("Hour hand color config", move |_| {set_hour_hand_color_toggle(!hour_hand_color_toggle)})
                .hotkey(VirtualKeyCode::W)
                .el(),
                if hour_hand_color_toggle {
                    FlowColumn::el([
                    row(
                        "Hour Red Value",
                        F32Input {
                            value: hour_hand_color_red,
                            on_change: set_hour_hand_color_red,
                        }
                        .el(),
                    ),
                    row(
                        "Hour Green Value",
                        F32Input {
                            value: hour_hand_color_green, 
                            on_change: set_hour_hand_color_green,
                        }
                        .el(),
                    ),
                    row(
                        "Hour Blue Value",
                        F32Input {
                            value: hour_hand_color_blue, 
                            on_change: set_hour_hand_color_blue,
                        }
                        .el(),
                    ),
                    row(
                        "Hour Alpha Value",
                        F32Input {
                            value: hour_hand_color_alpha,
                            on_change: set_hour_hand_color_alpha,
                        }
                        .el(),
                    )])
            } else {Element::new()},
            
            
            
            ])
            .with(translation(), vec3(window_width_for_ui - 150., 0., 0.))
            .with(width(), 400.)
            .with(space_between_items(), STREET)
            .with_padding_even(STREET),
        ]),
        drawing::draw_circle(clock_x_position, clock_y_position, clock_ray,
            Vec4{x:border_color_red, y:border_color_green, z:border_color_blue, w:border_color_alpha}),
        drawing::draw_static_hour_hand(clock_x_center, clock_y_center, hour_x, hour_y,
            Vec4{x:hour_hand_color_red, y:hour_hand_color_green, z:hour_hand_color_blue, w:hour_hand_color_alpha}),
        drawing::draw_static_minute_hand(clock_x_center, clock_y_center, minute_x, minute_y),
        drawing::draw_static_second_hand(clock_x_center, clock_y_center, second_x, second_y),
    ])
    
}


#[main]
pub fn main() {
    tests::color_tests();

    //With ambient-0.2.1, this test will fail is the parameter of it is different than zero.
    //Once the timezone bug of ambient is fixed, it should work if the tester inputs the correct timezone of its PC.
    tests::time_zone_test(0);
	start();
    
}

fn start(){
	App.el().spawn_interactive();
}

