use ambient_api::prelude::*;

fn CreateWhiteBackground() -> Element {
    Rectangle.el()
    .with(width(), 2000.)
    .with(height(), 2000.)
    .with(background_color(), vec4(0., 0., 0., 1.))
}

#[element_component]
fn App(_hooks: &mut Hooks) -> Element {
    Group::el([
        FlowColumn::el([
            Rectangle.el(),
            CreateWhiteBackground(),
        ]),
        Line.el()
            .with(line_from(), vec3(200., 200., 0.))
            .with(line_to(), vec3(300., 200., 0.))
            .with(line_width(), 1.)
            .with(background_color(), vec4(1., 0., 0., 1.)),
        Line.el()
            .with(line_from(), vec3(200., 200., 0.))
            .with(line_to(), vec3(200., 300., 0.))
            .with(line_width(), 1.)
            .with(background_color(), vec4(0., 1., 0., 1.)),
        Line.el()
            .with(line_from(), vec3(200., 200., 0.))
            .with(line_to(), vec3(500., 300., 0.))
            .with(line_width(), 10.)
            .with(background_color(), vec4(0., 0., 1., 1.)),
    ])
}

#[main]
pub fn main() {
    App.el().spawn_interactive();
}

