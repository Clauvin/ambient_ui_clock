use ambient_api::prelude::*;

fn CreateWhiteBackground(var_width: f32, var_height: f32) -> Element {
    Rectangle.el()
        .with(width(), var_width)
        .with(height(), var_height)
        .with(background_color(), vec4(1., 1., 1., 1.))
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
        CreateWhiteBackground(255., 255.),
    ])
}

#[element_component]
fn AppTwo(_hooks: &mut Hooks) -> Element {
    Group::el([
        DrawPoint(250., 250., 0.),
    ])
}


#[main]
pub fn main() {
    App.el().spawn_interactive();
    AppTwo.el().spawn_interactive();
}

