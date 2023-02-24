use nannou::color::*;
use nannou::draw::primitive::ellipse::Ellipse;
use nannou::draw::primitive::Path;
use nannou::draw::primitive::Rect as DrawRect;
use nannou::draw::*;
use nannou::prelude::*;
use regex::Regex;
use std::env;
use std::path::Path as SysPath;
use std::process;
use std::str::FromStr;

// Quick "spec" of env vars
// NOGO_COLOR - color of the lines being drawn
// NOGO_BG_COLOR - color of the canvas (default is transparent)
// NOGO_SHAPE - a placeholder for an eventual script (lol)
// NOGO_SIZE - how much of the canvas the shape should encompass, from 0.0-1.0
// NOGO_WEIGHT - the weight of the lines being drawn
// NOGO_ROTATION - an amount to rotate the shape in degrees
// NOGO_SHOW_TURTLE - when true, renders a little turtle (default: false)
// NOGO_OUTPUT - the path and file to export to (.png will be appended)

fn main() {
    nannou::app(model)
        .size(1000, 1000)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    count: i32,
    output: String,
}

fn model(_app: &App) -> Model {
    let output = env::var("NOGO_OUTPUT").unwrap_or("./nogo-draw".to_string());

    Model { count: 0, output }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Supremely sketch way to make sure the image has saved before
    // exiting the program.
    if model.count == 30 {
        process::exit(0);
    }

    if model.count == 0 {
        let path = SysPath::new(model.output.as_str()).with_extension("png");
        app.main_window().capture_frame(path);
    }

    model.count += 1;
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let line_color = color_from_env("NOGO_COLOR", "#60DEA9FF");
    let bg_color = color_from_env("NOGO_BG_COLOR", "#00000000");
    let size = float_from_env("NOGO_SIZE", 0.75);
    let weight = float_from_env("NOGO_WEIGHT", 4.0);
    let rotation = deg_to_rad(float_from_env("NOGO_ROTATION", 0.0));

    let draw = app.draw();
    let boundary = app.window_rect();

    draw.background().color(bg_color);

    match env::var("NOGO_SHAPE")
        .unwrap_or("hexagon".to_string())
        .as_str()
    {
        "hexagon" => {
            polygon(6, &draw, boundary, size, weight, line_color, rotation);
        }
        "triangle" => {
            polygon(3, &draw, boundary, size, weight, line_color, rotation);
        }
        "square" => {
            square(&draw, boundary, size)
                .stroke_weight(weight)
                .stroke_color(line_color)
                .rotate(rotation);
        }
        "circle" => {
            circle(&draw, boundary, size)
                .stroke_weight(weight)
                .stroke_color(line_color)
                .rotate(rotation);
        }
        _ => panic!("Unsupported shape type, choose one of [circle, triangle, square, hexagon]"),
    }

    draw.to_frame(app, &frame).unwrap();
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

// Circle
fn circle(draw: &Draw, boundary: Rect, size: f32) -> Drawing<Ellipse> {
    draw.ellipse()
        .width(boundary.w() * size)
        .height(boundary.h() * size)
        .no_fill()
}

// Square
fn square(draw: &Draw, boundary: Rect, size: f32) -> Drawing<DrawRect> {
    draw.rect()
        .width(boundary.w() * size)
        .height(boundary.h() * size)
        .no_fill()
}

// Poly
fn polygon(
    sides: usize,
    draw: &Draw,
    boundary: Rect,
    size: f32,
    weight: f32,
    color: Rgba<u8>,
    rotation: f32,
) -> Drawing<Path> {
    if sides == 7 {
        panic!("Seven sides?? Don't be ridiculous");
    }

    let radius = boundary.w() * size / 2.0;
    let points = (0..=360).step_by(360 / sides).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * radius;
        let y = radian.cos() * radius;
        pt2(x, y)
    });
    draw.polyline()
        .weight(weight)
        .color(color)
        .points(points)
        .rotate(rotation)
}
