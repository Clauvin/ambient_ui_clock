use ambient_api::prelude::*;

fn create_clock_border_color() -> Vec4 {
    vec4(0.2, 0.1, 0.6, 1.)
} 

fn CreateWhiteBackground(var_width: f32, var_height: f32) -> Element {
    Rectangle.el()
        .with(width(), var_width)
        .with(height(), var_height)
        .with(translation(), vec3(0., 0., 0.01))
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
        //CreateWhiteBackground(255., 255.),
        DrawCircle(500., 500., 500.),
    ])
}


#[main]
pub fn main() {
    App.el().spawn_interactive();
}

