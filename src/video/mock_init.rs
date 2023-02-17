#![cfg(not(target_arch = "aarch64"))]

use anyhow::Result;
use embedded_graphics::{prelude::*, pixelcolor::Rgb565};
use embedded_graphics_simulator::{SimulatorDisplay, OutputSettingsBuilder, Window, SimulatorEvent};
use lvgl::LvError;
use super::gui::load_gui;

pub fn mock_init_fb() -> Result<(), LvError> {
    // For some reason Rgb888 is broken. TODO: Figure out why this is
    let mock_display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(1080, 2160));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Test", &output_settings);
    // Some(window) for code crimes. See init.rs:10
    load_gui(mock_display, Some(window))?;
    Ok(())
}