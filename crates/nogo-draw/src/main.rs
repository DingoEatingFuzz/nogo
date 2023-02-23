use nannou::color::*;
use nannou::prelude::*;
use regex::Regex;
use std::env;
use std::str::FromStr;

// Quick "spec" of env vars
// NOGO_COLOR - color of the lines being drawn
// NOGO_BG_COLOR - color of the canvas (default is transparent)
// NOGO_SHAPE - a placeholder for an eventual script (lol)
// NOGO_SIZE - how much of the canvas the shape should encompass, from 0.0-1.0
// NOGO_WEIGHT - the weight of the lines being drawn
// NOGO_SHOW_TURTLE - when true, renders a little turtle (default: false)

fn main() {
    nannou::app(model).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let line_color = color_from_env("NOGO_COLOR", "#60DEA9FF");
    let bg_color = color_from_env("NOGO_BG_COLOR", "#00000000");
    let size = float_from_env("NOGO_SIZE", 0.75);
    let weight = float_from_env("NOGO_WEIGHT", 4.0);

    let draw = app.draw();
    let boundary = app.window_rect();

    draw.background().color(bg_color);
    draw.line()
        .start(pt2(-boundary.w() / 2.0 * size, boundary.h() / 2.0 * size))
        .end(pt2(boundary.w() / 2.0 * size, -boundary.h() / 2.0 * size))
        .weight(weight)
        .color(line_color);

    draw.to_frame(app, &frame).unwrap();

    let path = app
        .project_path()
        .expect("no project_path?")
        .join(app.exe_name().unwrap())
        .with_extension("png");

    app.main_window().capture_frame(path)
}

fn color_from_env(env_var: &str, default_val: &str) -> Rgba<u8> {
    let value = env::var(env_var).unwrap_or(default_val.to_string());
    let re = Regex::new(r"^#[\dA-Fa-f]{8}$").unwrap();
    if !re.is_match(&value) {
        panic!("Invalid color '{}' found for ENV VAR {}", value, env_var);
    }

    rgba_u32(u32::from_str_radix(value.trim_start_matches("#"), 16).unwrap())
}

fn rgba_u32(hex: u32) -> Rgba<u8> {
    // Nannou ships with an rgb_u32 function just like this one
    // except it doesn't support an alpha channel :(
    let alpha: u8 = (hex & 0xFF) as u8;
    let blue: u8 = ((hex >> 8) & 0xFF) as u8;
    let green: u8 = ((hex >> 16) & 0xFF) as u8;
    let red: u8 = ((hex >> 24) & 0xFF) as u8;
    rgba(red, green, blue, alpha)
}

fn float_from_env(env_var: &str, default_val: f32) -> f32 {
    let value = env::var(env_var);
    match value {
        Ok(val) => f32::from_str(&val).expect("float_from_env received a non-float value from ENV"),
        Err(_error) => default_val,
    }
}
