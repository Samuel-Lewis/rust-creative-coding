use clap::Clap;
use nannou::noise::*;
use nannou::prelude::*;
use std::env;

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Opts {
    #[clap(short, long)]
    fps: bool,
}

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    options: Opts,
    perlin: Perlin,
    mode: usize,
    modes: [fn(&App, &Model, f32) -> f32; 4],
}

fn model(app: &App) -> Model {
    app.new_window()
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();

    Model {
        options: Opts::parse(),
        perlin: Perlin::new(),
        mode: 0,
        modes: [mode_circle, mode_sin, mode_noise, mode_random],
    }
}

fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    model.mode = (model.mode + 1) % model.modes.len();
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn mode_circle(_app: &App, _model: &Model, _radian: f32) -> f32 {
    0.0
}

fn mode_sin(app: &App, _model: &Model, radian: f32) -> f32 {
    let amp = map_range(
        app.mouse.position().y,
        app.window_rect().bottom(),
        app.window_rect().top(),
        3,
        10,
    );

    let d = rad_to_deg(radian);
    let d = d * amp as f32 + (app.elapsed_frames() as f32);
    let d = deg_to_rad(d);
    map_range(d.sin(), -1.0, 1.0, 0.0, 25.0)
}

fn mode_random(app: &App, _model: &Model, _radian: f32) -> f32 {
    let amp = map_range(
        app.mouse.position().y,
        app.window_rect().bottom(),
        app.window_rect().top(),
        5.0,
        100.0,
    );
    random_range(0.0, amp)
}

fn mode_noise(app: &App, model: &Model, radian: f32) -> f32 {
    let x = radian.sin();
    let y = radian.cos();

    let amp = map_range(
        app.mouse.position().y,
        app.window_rect().bottom(),
        app.window_rect().top(),
        1.0,
        150.0,
    );

    let noise = model
        .perlin
        .get([x as f64, y as f64, app.elapsed_frames() as f64 * 0.01]);

    return map_range(noise, -1.0, 1.0, 0.0, amp) as f32;
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();
    let win = app.window_rect();

    // Print FPS
    if model.options.fps {
        draw.text(format!("{:.0}", app.fps()).as_str())
            .color(RED)
            .xy(win.top_left() + pt2(30.0, -30.0));
    }

    let radius = map_range(
        app.mouse.position().x,
        win.left(),
        win.right(),
        1.0,
        win.w() / 4.0,
    );

    let points = (0..=360).map(|i| {
        let radian = deg_to_rad(i as f32);
        let offset = model.modes[model.mode](&app, &model, radian) + radius;

        let x = radian.sin() * offset;
        let y = radian.cos() * offset;

        (pt2(x, y), STEELBLUE)
    });

    draw.polyline().weight(3.0).points_colored(points);

    draw.to_frame(app, &frame).unwrap();
}
