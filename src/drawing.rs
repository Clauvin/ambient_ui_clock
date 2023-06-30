use ambient_api::prelude::*;

pub const CLOCK_BORDER_COLOR: Vec4 = vec4(0.2, 0.1, 0.6, 1.);
pub const HOUR_COLOR: Vec4 = CLOCK_BORDER_COLOR;
pub const MINUTE_COLOR: Vec4 = vec4(0.2, 0.6, 0.1, 1.);
pub const SECOND_COLOR: Vec4 = vec4(0.6, 0.1, 0.2, 1.);

pub fn draw_circle(x_position: f32, y_position: f32, ray: f32, circle_border_color: Vec4) -> Element{
    Rectangle.el()
        .with(width(), ray*2.)
        .with(height(), ray*2.)
        .with(translation(), vec3(x_position-ray, y_position-ray, 0.01))
        .with(border_color(), circle_border_color)
        .with(border_thickness(), 4.)
        .with(border_radius(), vec4(ray,ray,ray,ray))
}

pub fn draw_hand(from_x: f32, from_y:f32, to_x: f32, to_y: f32, hand_color: Vec4) -> Element {
    Line.el()
    .with(line_from(), vec3(from_x, from_y, 0.0))
    .with(line_to(), vec3(to_x, to_y, 0.0))
    .with(line_width(), 4.)
    .with(background_color(), hand_color)
}

pub fn draw_static_hour_hand(from_x: f32, from_y:f32, to_x: f32, to_y: f32, color: Vec4) -> Element {
    draw_hand(from_x, from_y, to_x, to_y, color)
}

pub fn draw_static_minute_hand(from_x: f32, from_y:f32, to_x: f32, to_y: f32) -> Element {
    draw_hand(from_x, from_y, to_x, to_y, MINUTE_COLOR)
}

pub fn draw_static_second_hand(from_x: f32, from_y:f32, to_x: f32, to_y: f32) -> Element {
    draw_hand(from_x, from_y, to_x, to_y, SECOND_COLOR)
}

pub fn define_clock_center(window_width: u32, window_height: u32) -> f32{
    if window_width <= window_height{
        (window_width/2) as f32
    } else {
        (window_height/2) as f32
    }
}