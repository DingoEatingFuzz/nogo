use nannou::color::*;
use nannou::prelude::*;
use regex::Regex;
use std::env;

fn main() {
    nannou::app(model).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let line_color = color_from_env("NOGO_COLOR");
    let draw = app.draw();
    let boundary = app.window_rect();

    draw.background().color(PURPLE);
    draw.line()
        .start(pt2(-boundary.w() / 2.0, boundary.h() / 2.0))
        .end(pt2(boundary.w() / 2.0, -boundary.h() / 2.0))
        .weight(4.0)
        .color(line_color);

    draw.to_frame(app, &frame).unwrap();

    let path = app
        .project_path()
        .expect("no project_path?")
        .join(app.exe_name().unwrap())
        .with_extension("png");

    app.main_window().capture_frame(path)
}

fn color_from_env(env_var: &str) -> Rgba<u8> {
    let value = env::var(env_var).unwrap_or("#000000FF".to_string());
    let re = Regex::new(r"^#[\dA-Fa-f]{8}$").unwrap();
    if !re.is_match(&value) {
        panic!("Invalid color '{}' found for ENV VAR {}", value, env_var);
    }

    rgba_u32(u32::from_str_radix(value.trim_start_matches("#"), 16).unwrap())
}

fn rgba_u32(hex: u32) -> Rgba<u8> {
    let alpha: u8 = (hex & 0xFF) as u8;
    let blue: u8 = ((hex >> 8) & 0xFF) as u8;
    let green: u8 = ((hex >> 16) & 0xFF) as u8;
    let red: u8 = ((hex >> 24) & 0xFF) as u8;
    rgba(red, green, blue, alpha)
}
