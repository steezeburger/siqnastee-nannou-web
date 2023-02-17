use std::cell::RefCell;

use nannou::prelude::*;
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};

use rand::distributions::{Alphanumeric, DistString};

pub struct Model {
    /// The width of the browser window
    pub width: f32,

    /// The height of the browser window
    pub height: f32,
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

// TODO - regenerate view on resize
fn resized(_app: &App, _model: &mut Model, window_size: Vec2) {
    // FIXME - doesn't seem to work. is it because we're in a browser?
    web_sys::console::log_1(
        &format!("w: {}, h: {}", window_size.x, window_size.y).into());
}

// TODO - regenerate view on touch
fn touch(_app: &App, _model: &mut Model, _touch: TouchEvent) {}

// TODO - regenerate view on mouse move
fn mouse_moved(_app: &App, _model: &mut Model, _: Point2) {}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(BLACK);

    // Where the actual drawing happens
    draw_main(&draw, model.width, model.height);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn draw_main(draw: &Draw, width: f32, height: f32) {
    web_sys::console::log_1(&format!("w: {}, h: {}", width, height).into());

    let rand_letter = Alphanumeric.sample_string(&mut rand::thread_rng(), 1);

    let rect_w: f32 = 5.0;
    let rect_h: f32 = 10.0;

    let top_left = pt2(-width / 2.0 + 50.0, height / 2.0 + 50.0);
    let offset = vec2(rect_w / 2.0, -rect_h / 2.0);
    let xy = top_left + offset;

    draw.rect()
        .xy(xy)
        .w_h(rect_w, rect_h)
        .color(SPRINGGREEN);

    draw.text(&rand_letter)
        .x(0.0)
        .y(0.0)
        .font_size(24)
        .color(REBECCAPURPLE);
}


pub async fn run_app(model: Model) {
    // Since ModelFn is not a closure we need this workaround to pass the calculated model
    thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());

    MODEL.with(|m| m.borrow_mut().replace(model));

    app::Builder::new_async(|app| {
        Box::new(async move {
            create_window(app).await;
            MODEL.with(|m| m.borrow_mut().take().unwrap())
        })
    })
        .backends(Backends::PRIMARY | Backends::GL)
        .update(update)
        .run_async()
        .await;
}

async fn create_window(app: &App) {
    let device_desc = DeviceDescriptor {
        limits: Limits {
            max_texture_dimension_2d: 8192,
            ..Limits::downlevel_webgl2_defaults()
        },
        ..Default::default()
    };

    app.new_window()
        .device_descriptor(device_desc)
        .title("siqnastee")
        .mouse_moved(mouse_moved)
        // TODO
        // .touch(touch)
        // .resized(resized)
        .view(view)
        .build_async()
        .await
        .unwrap();
}
