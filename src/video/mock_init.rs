#![cfg(not(target_arch = "aarch64"))]

use std::{thread::sleep, time::Duration};

use anyhow::Result;
use embedded_graphics::{prelude::*, pixelcolor::Rgb888};
use embedded_graphics_simulator::{SimulatorDisplay, OutputSettingsBuilder, Window, SimulatorEvent};
use lvgl::{UI, LvError, style::Style, State, Color, Widget, Part};
use super::gui::load_gui;

pub fn mock_init_fb() -> Result<(), LvError> {
    let mock_display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(1080, 2160));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Test", &output_settings);
    let mut ui: UI<SimulatorDisplay<Rgb888>, _> = UI::init()?;
    ui.disp_drv_register(mock_display).unwrap();
    let mut screen = ui.scr_act()?;
    let mut screen_style = Style::default();
    screen_style.set_bg_color(State::DEFAULT, Color::from_rgb((0, 0, 0)));
    screen_style.set_radius(State::DEFAULT, 0);
    screen.add_style(Part::Main, screen_style);
    'running: loop {
        window.update(ui.get_display_ref().unwrap());
        sleep(Duration::from_secs(1));
        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
    }
    //load_gui(mock_display);
    Ok(())
}