use nannou::prelude::*;

pub trait Paint {
    fn paint(&self, app: &App, draw: &Draw);
}

pub trait Think {
    fn think(&mut self, app: &App);
}

#[derive(Debug, Clone)]
pub struct Star {
    pub point: Vector3<f32>,
    pub last_z: f32,
}

impl Star {
    pub fn start_pos(app: &App) -> Vector3<f32> {
        Vector3::new(
            random_range(app.window_rect().left(), app.window_rect().right()),
            random_range(app.window_rect().top(), app.window_rect().bottom()),
            random_range(0.0, app.window_rect().w()),
        )
    }

    pub fn new(app: &App) -> Star {
        let p = Star::start_pos(&app);
        Star {
            point: p,
            last_z: p.z,
        }
    }
}

impl Paint for Star {
    fn paint(&self, app: &App, draw: &Draw) {
        let x = self.point.x;
        let y = self.point.y;
        let z = self.point.z;
        let last_z = self.last_z;

        let px = map_range(
            x / last_z,
            -1.0,
            1.0,
            app.window_rect().left(),
            app.window_rect().right(),
        );

        let py = map_range(
            y / last_z,
            -1.0,
            1.0,
            app.window_rect().top(),
            app.window_rect().bottom(),
        );

        let sx = map_range(
            x / z,
            -1.0,
            1.0,
            app.window_rect().left(),
            app.window_rect().right(),
        );

        let sy = map_range(
            y / z,
            -1.0,
            1.0,
            app.window_rect().top(),
            app.window_rect().bottom(),
        );

        let r = map_range(z, 0.0, app.window_rect().w(), 2.0, 0.0);

        draw.ellipse().color(STEELBLUE).radius(r).x_y(sx, sy);

        let draw_line = true;
        if draw_line {
            draw.line()
                .start(pt2(px, py))
                .end(pt2(sx, sy))
                .weight(r)
                .color(WHITE);
        }
    }
}

impl Think for Star {
    fn think(&mut self, app: &App) {
        self.last_z = self.point.z;
        self.point.z -= map_range(
            app.mouse.position().x,
            app.window_rect().left(),
            app.window_rect().right(),
            -5.0,
            30.0,
        );
        if self.point.z < 1.0 {
            self.point = Star::start_pos(&app);
            self.point.z = app.window_rect().w();
            self.last_z = self.point.z;
        }

        if self.point.z > app.window_rect().w() {
            self.point = Star::start_pos(&app);
            self.point.z = 1.0;
            self.last_z = self.point.z;
        }
    }
}
