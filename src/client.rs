use ambient_api::prelude::*;

const clock_x_position: f32 = 500.;
const clock_y_position: f32 = 375.;
const clock_ray: f32 = 500.;

fn create_clock_border_color() -> Vec4 {
    vec4(0.2, 0.1, 0.6, 1.)
} 


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

fn DrawCircle(x_position: f32, y_position: f32, ray: f32) -> Element{
    Rectangle.el()
        .with(width(), ray/2.)
        .with(height(), ray/2.)
        .with(translation(), vec3(x_position-ray/2., y_position-ray/2., 0.01))
        .with(border_color(), create_clock_border_color())
        .with(border_thickness(), 4.)
        .with(border_radius(), vec4(ray/4.,ray/4.,ray/4.,ray/4.))
}

fn DrawHand(x: f32, y: f32) -> Element {
    Line.el()
    .with(line_from(), vec3(x, y, 0.0))
    .with(line_to(), vec3(x, y-30., 0.0))
    .with(line_width(), 4.)
    .with(background_color(), vec4(0.6, 0.2, 0.2, 1.))
}

fn DrawStaticHourHand(x: f32, y: f32) -> Element {
    DrawHand(x, y)
}


#[element_component]
fn App(_hooks: &mut Hooks) -> Element {
    Group::el([
        CreateWhiteBackground(_hooks),
        DrawCircle(clock_x_position, clock_y_position, clock_ray),
        DrawStaticHourHand(clock_x_position-clock_ray/4., clock_y_position-clock_ray/4.)
    ])
}


#[main]
pub fn main() {
    App.el().spawn_interactive();
}

