#![cfg(not(target_arch = "aarch64"))]

use anyhow::Result;
use embedded_graphics::{prelude::*, pixelcolor::Rgb888};
use embedded_graphics_simulator::{SimulatorDisplay, OutputSettingsBuilder, Window};
use lvgl::{UI, LvError};
use super::gui::load_gui;

pub fn mock_init_fb() -> Result<(), LvError> {
    let mock_display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(1080, 2160));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Test", &output_settings);
    let mut ui = UI::init()?;
    ui.disp_drv_register(mock_display).unwrap();

    load_gui(mock_display);
    Ok(())
}