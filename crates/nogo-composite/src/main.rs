use nannou::prelude::*;
use std::env;
use std::fs;
use std::process;

fn main() {
    nannou::app(model)
        .size(1000, 1000)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    textures: Vec<wgpu::Texture>,
    count: i32,
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Supremely sketch way to make sure the image has saved before
    // exiting the program.
    if model.count == 30 {
        process::exit(0);
    }

    if model.count == 0 {
        let path = app
            .project_path()
            .expect("no project_path?")
            .join(app.exe_name().unwrap())
            .with_extension("png");

        app.main_window().capture_frame(path);
    }

    model.count += 1;
}

fn model(app: &App) -> Model {
    let img_path =
        env::var("NOGO_DIR").expect("Must define a NOGO_DIR ENV VAR to read images from");
    let paths = fs::read_dir(img_path).expect("Non-existent directory provided");

    let mut textures = Vec::new();
    for entry in paths {
        textures.push(wgpu::Texture::from_path(app, entry.unwrap().path()).unwrap());
    }

    Model { textures, count: 0 }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();
    let boundary = app.window_rect();

    for texture in &model.textures {
        draw.texture(texture).wh(boundary.wh());
    }

    draw.to_frame(app, &frame).unwrap();
}
