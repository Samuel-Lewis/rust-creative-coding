mod star;
use clap::Clap;
use nannou::prelude::*;
use star::*;
use std::env;

const MAX_STARS: u32 = 400;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Opts {
    #[clap(short, long)]
    fps: bool,
}

// let app = App::new(env!("CARGO_PKG_NAME"))
// .author(env!("CARGO_PKG_AUTHORS"))
// .version(env!("CARGO_PKG_VERSION"))
fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    stars: Vec<Star>,
    show_fps: bool,
}

fn model(app: &App) -> Model {
    let opts: Opts = Opts::parse();
    let mut stars: Vec<Star> = Vec::new();

    for _ in 1..MAX_STARS {
        stars.push(Star::new(app));
    }

    Model {
        stars,
        show_fps: opts.fps,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for star in model.stars.iter_mut() {
        star.think(app);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    // Print FPS
    let draw = app.draw();
    if model.show_fps {
        draw.text(format!("{:.0}", app.fps()).as_str())
            .color(RED)
            .xy(app.window_rect().top_left() + pt2(30.0, -30.0));
    }

    for star in model.stars.iter() {
        star.paint(&app, &draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
