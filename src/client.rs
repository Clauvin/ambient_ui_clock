use ambient_api::prelude::*;

fn CreateWhiteBackground() -> Element {
    Rectangle.el()
        .with(width(), 500.)
        .with(height(), 500.)
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
        
        DrawPoint(250., 250., 0.),
        //CreateWhiteBackground(),
    ])
}

#[main]
pub fn main() {
    App.el().spawn_interactive();
}

