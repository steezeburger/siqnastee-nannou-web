use std::cell::RefCell;

use nannou::color::IntoLinSrgba;
use nannou::draw::properties::ColorScalar;
use nannou::prelude::*;
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;

static RECTANGLE_WIDTH: f32 = 16.0;
static HALF_RECTANGLE_WIDTH: f32 = RECTANGLE_WIDTH / 2.0;
static RECTANGLE_HEIGHT: f32 = 24.0;
static HALF_RECTANGLE_HEIGHT: f32 = RECTANGLE_HEIGHT / 2.0;

pub struct Model {
    /// width of the window
    window_width: f32,
    /// height of the window
    window_height: f32,
    /// number of columns
    num_cols: usize,
    /// number of rows
    num_rows: usize,
    /// the grid of rectangles
    grid: Vec<Vec<SiqRect>>,
}

#[derive(Clone)]
pub struct SiqRect {
    /// The color this rectangle should be
    color: Rgb,

    /// The Nannou Rect
    rect: Rect,

    /// True if we've manually set this rect's color
    touched: bool,
}

impl Model {
    fn new(app: &App) -> Self {
        let window_width = app.window_rect().w();
        let window_height = app.window_rect().h();

        // The size of our rectangles.
        let rectangle_width = RECTANGLE_WIDTH;
        let rectangle_height = RECTANGLE_HEIGHT;

        // Calculate the number of columns and rows needed to fill the window.
        let num_cols = (window_width / rectangle_width).ceil() as usize;
        let num_rows = (window_height / rectangle_height).ceil() as usize;

        // Calculate the x and y positions of the center of the grid.
        let center_x = app.window_rect().left() + app.window_rect().w() / 2.0;
        let center_y = app.window_rect().bottom() + app.window_rect().h() / 2.0;

        // Calculate the starting x and y positions for the grid.
        let start_x = HALF_RECTANGLE_WIDTH + center_x - (num_cols as f32 / 2.0) * RECTANGLE_WIDTH;
        let start_y = HALF_RECTANGLE_HEIGHT + center_y - (num_rows as f32 / 2.0) * RECTANGLE_HEIGHT;

        // Create the grid that will be rendered later.
        let default_rect = SiqRect {
            color: Rgb::new(0.0, 0.0, 0.0),
            rect: Rect::from_w_h(0.0, 0.0),
            touched: false,
        };

        let mut grid = vec![vec![default_rect; num_rows]; num_cols];
        for i in 0..num_cols {
            for j in 0..num_rows {
                let x = start_x + i as f32 * rectangle_width;
                let y = start_y + j as f32 * rectangle_height;
                let rect = SiqRect {
                    color: get_random_color(),
                    rect: Rect::from_x_y_w_h(x, y, rectangle_width, rectangle_height),
                    touched: false,
                };
                grid[i][j] = rect;
            }
        }
        Self {
            num_cols,
            num_rows,
            window_width,
            window_height,
            grid,
        }
    }
}

fn model(app: &App) -> Model {
    Model::new(app)
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for row in &model.grid {
        for siq_rect in row {
            // draw rectangle black by default or with a random color if it's been touched
            let mut color = Rgb::new(0.0, 0.0, 0.0);
            if siq_rect.touched {
                color = get_random_color();
            }
            draw.rect()
                .xy(siq_rect.rect.xy())
                .w_h(siq_rect.rect.w(), siq_rect.rect.h())
                .color(color);

            // draw random letter
            let rand_letter = Alphanumeric.sample_string(&mut rand::thread_rng(), 1);
            draw.text(&rand_letter)
                .xy(siq_rect.rect.xy())
                .font_size(12)
                .color(get_random_color());
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MouseMoved(location) => {
            let rand_x = rand::thread_rng().gen_range(0..model.num_cols);
            let rand_y = rand::thread_rng().gen_range(0..model.num_rows);
            model.grid[rand_x][rand_y].touched = true;
        }
        _ => {}
    }
}

/// Return random Rgb color
fn get_random_color() -> Rgb {
    let red = rand::random();
    let green = rand::random();
    let blue = rand::random();
    Rgb::new(red, green, blue)
}

pub async fn run_app(width: u32, height: u32) {
    // Since ModelFn is not a closure we need this workaround to pass the calculated model
    thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());

    app::Builder::new_async(move |app| {
        Box::new(async move {
            create_window(app, width, height).await;
            let model = Model::new(app);
            MODEL.with(|m| m.borrow_mut().replace(model));
            MODEL.with(|m| m.borrow_mut().take().unwrap())
        })
    })
        .backends(Backends::PRIMARY | Backends::GL)
        // .update(update)
        .run_async()
        .await;
}

async fn create_window(app: &App, width: u32, height: u32) {
    let device_desc = DeviceDescriptor {
        limits: Limits {
            max_texture_dimension_2d: 8192,
            ..Limits::downlevel_webgl2_defaults()
        },
        ..Default::default()
    };

    app.new_window()
        .size(width, height)
        .device_descriptor(device_desc)
        .title("siqnastee")
        // TODO
        // .mouse_moved(mouse_moved)
        // .touch(touch)
        // .resized(resized)
        .view(view)
        .event(event)
        .build_async()
        .await
        .unwrap();
}
