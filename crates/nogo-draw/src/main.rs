use nannou::prelude::*;

fn main() {
    nannou::app(model).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PURPLE);

    let boundary = app.window_rect();
    draw.line()
        .start(pt2(-boundary.w() / 2.0, boundary.h() / 2.0))
        .end(pt2(boundary.w() / 2.0, -boundary.h() / 2.0))
        .weight(4.0)
        .color(WHITE);

    draw.to_frame(app, &frame).unwrap();

    let path = app
        .project_path()
        .expect("no project_path?")
        .join(app.exe_name().unwrap())
        .with_extension("png");
    app.main_window().capture_frame(path);
}
