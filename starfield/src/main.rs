mod star;
use clap::Clap;
use nannou::prelude::*;
use star::*;
use std::env;

const MAX_STARS: u32 = 400;

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Opts {
    #[clap(short, long)]
    fps: bool,
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    stars: Vec<Star>,
    options: Opts,
}

fn model(app: &App) -> Model {
    let mut stars: Vec<Star> = Vec::new();

    for _ in 1..MAX_STARS {
        stars.push(Star::new(app));
    }

    Model {
        stars,
        options: Opts::parse(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for star in model.stars.iter_mut() {
        star.think(app);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();

    // Print FPS
    if model.options.fps {
        draw.text(format!("{:.0}", app.fps()).as_str())
            .color(RED)
            .xy(app.window_rect().top_left() + pt2(30.0, -30.0));
    }

    for star in model.stars.iter() {
        star.paint(&app, &draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
