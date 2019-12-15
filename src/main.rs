extern crate nannou;

use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    points: Vec<Vector2>,
    time: f32,
}

fn model(_app: &App) -> Model {
    Model {
        time: 0.,
        points: Vec::new(),
    }
}

// By default, `update` is called right before `view` is called each frame.
fn update(app: &App, model: &mut Model, _update: Update) {
    model.time += 0.1;
    model.points.push(
        Vector2::new(
            model.time as f32 * 10.,
            (model.time as f64 * 10.0).sin() as f32,
        ) * 10.0,
    );
}

fn view(app: &App, model: &Model, frame: &Frame) {
    // Begin drawing.
    let draw = app.draw();
    // Draw dark gray for the background
    draw.background().color(BLACK);

    // Let's graph a line maybe?
    for point in &model.points {
        draw.ellipse().color(WHITE).w(2.0).h(2.0).xy(point.clone());
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
