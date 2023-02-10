use nannou::{color::rgb::Rgb, prelude::*};
use std::collections::VecDeque;

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
    next_colors: VecDeque<Target>,
    draw_text: bool,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            position: Default::default(),
            color: Default::default(),
            next_colors: Default::default(),
            draw_text: false,
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

        if self.draw_text {
            draw.text("Pressed")
                .color(BLACK)
                .font_size(24)
                .xy(self.position + Vec2::new(0.0, 70.0));
        }
    }

    fn change_color(&mut self, color: Rgb, draw_text: bool) {
        self.draw_text = draw_text;
        self.next_colors.push_front(Target::Now(color))
    }

    fn change_color_delayed(&mut self, color: Rgb, target_frame: u64, draw_text: bool) {
        self.draw_text = draw_text;
        self.next_colors
            .push_front(Target::Future(color, target_frame));
    }

    fn update(&mut self, current_frame: u64) {
        if let Some(next_color) = self.next_colors.pop_back() {
            match next_color {
                Target::Now(color) => self.color = color,
                Target::Future(color, target_frame) => {
                    if current_frame >= target_frame {
                        self.color = color;
                    } else {
                        self.next_colors.push_back(next_color);
                    }
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
    app.set_loop_mode(LoopMode::rate_fps(60.0));

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
    let slack = (fps / 10.0) as u64;
    let target_frame = app.elapsed_frames() + slack;

    match event {
        KeyPressed(key) => match key {
            Key::F => {
                model.left_body.change_color(Rgb::new(1.0, 0.1, 0.1), true);
            }
            Key::J => {
                model
                    .right_body
                    .change_color_delayed(Rgb::new(1.0, 0.1, 0.1), target_frame, true);
            }
            _ => {}
        },
        KeyReleased(key) => match key {
            Key::F => {
                model.left_body.change_color(Rgb::new(0.0, 0.0, 0.0), false);
            }
            Key::J => {
                model
                    .right_body
                    .change_color_delayed(Rgb::new(0.0, 0.0, 0.0), target_frame, false);
            }
            _ => {}
        },
        _ => {}
    }
}
