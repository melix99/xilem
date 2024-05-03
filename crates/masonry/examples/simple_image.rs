// Copyright 2020 the Xilem Authors and the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! This showcase demonstrates how to use the image widget and is
//! properties. You can change the parameters in the GUI to see how
//! everything behaves.

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use masonry::app_driver::{AppDriver, DriverCtx};
use masonry::widget::{FillStrat, Image};
use masonry::{Action, WidgetId};
use vello::peniko::{Format, Image as ImageBuf};
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::Window;

struct Driver;

impl AppDriver for Driver {
    fn on_action(&mut self, _ctx: &mut DriverCtx<'_>, _widget_id: WidgetId, _action: Action) {}
}

pub fn main() {
    let image_bytes = include_bytes!("./assets/PicWithAlpha.png");
    let image_data = image::load_from_memory(image_bytes).unwrap().to_rgba8();
    let (width, height) = image_data.dimensions();
    let png_data = ImageBuf::new(image_data.to_vec().into(), Format::Rgba8, width, height);
    let image = Image::new(png_data).fill_mode(FillStrat::Contain);

    let event_loop = EventLoop::new().unwrap();
    let window_size = LogicalSize::new(650.0, 450.0);
    #[allow(deprecated)]
    let window = event_loop
        .create_window(
            Window::default_attributes()
                .with_title("Simple image example")
                .with_min_inner_size(window_size)
                .with_max_inner_size(window_size),
        )
        .unwrap();

    masonry::event_loop_runner::run(image, window, event_loop, Driver).unwrap();
}