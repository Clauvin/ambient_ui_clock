use ambient_api::prelude::*;

fn create_clock_border_color() -> Vec4 {
    vec4(0.2, 0.1, 0.6, 1.)
} 

fn CreateWhiteBackground(_hooks: &mut Hooks) -> Element {
    let size_info = _hooks.use_query(window_logical_size());
    let x = size_info[0].1.x as f32;
    let y = size_info[0].1.y as f32;
    Rectangle.el()
        .with(width(), x)
        .with(height(), y)
        .with(background_color(), vec4(1., 1., 1., 1.))
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

fn DrawPoint(x: f32, y: f32, depth: f32) -> Element {
    Line.el()
        .with(line_from(), vec3(x, y, depth))
        .with(line_to(), vec3(x+1., y+1., depth))
        .with(line_width(), 1.)
        .with(background_color(), vec4(0., 1., 0., 1.))
}

#[element_component]
fn App(_hooks: &mut Hooks) -> Element {
    Group::el([
        //CreateWhiteBackground(_hooks),
        DrawCircle(500., 375., 500.),
    ])
}


#[main]
pub fn main() {
    App.el().spawn_interactive();
}

