use nannou::{color::rgb::Rgb, prelude::*};

fn main() {
    nannou::app(model).update(update).run();
}

enum Target {
    Now(Rgb),
    Future(Rgb, u64),
}

struct Body {
    position: Vec2,
    color: Rgb,
    next_color: Target,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            color: Rgb::new(0.0, 0.0, 0.0),
            next_color: Target::Now(Rgb::new(0.0, 0.0, 0.0)),
        }
    }
}

impl Body {
    fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            ..Body::default()
        }
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .radius(30.0)
            .color(self.color)
            .xy(self.position);
    }

    fn change_color(&mut self, color: Rgb) {
        self.next_color = Target::Now(color)
    }

    fn change_color_delayed(&mut self, color: Rgb, target_frame: u64) {
        self.next_color = Target::Future(color, target_frame);
    }

    fn update(&mut self, current_frame: u64) {
        match self.next_color {
            Target::Now(color) => {
                self.color = color;
            }
            Target::Future(color, target_frame) => {
                if current_frame == target_frame {
                    self.color = color;
                }
            }
        }
    }
}

struct Model {
    left_body: Body,
    right_body: Body,
}

fn model(app: &App) -> Model {
    app.new_window().event(event).view(view).build().unwrap();

    Model {
        left_body: Body::new(-32.0, 0.0),
        right_body: Body::new(32.0, 0.0),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if app.elapsed_frames() % 100 == 0 {
        println!("FPS {:?}", app.fps());
    }

    model.left_body.update(app.elapsed_frames());
    model.right_body.update(app.elapsed_frames());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().rgb(0.98, 0.96, 0.94);

    // Left side circle
    model.left_body.draw(&draw);

    // Right side circle
    model.right_body.draw(&draw);

    // Draw to frame
    draw.to_frame(app, &frame).unwrap();
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    let fps = app.fps();
    let slack = (fps / 10.0) as u64 - 1;
    let target_frame = app.elapsed_frames() + slack;

    match event {
        KeyPressed(key) => match key {
            Key::F => {
                model.left_body.change_color(Rgb::new(0.3, 0.3, 0.3));
            }
            Key::J => {
                model
                    .right_body
                    .change_color_delayed(Rgb::new(0.3, 0.3, 0.3), target_frame);
            }
            _ => {}
        },
        KeyReleased(key) => match key {
            Key::F => {
                model.left_body.change_color(Rgb::new(0.0, 0.0, 0.0));
            }
            Key::J => {
                model
                    .right_body
                    .change_color_delayed(Rgb::new(0.0, 0.0, 0.0), target_frame);
            }
            _ => {}
        },
        _ => {}
    }
}