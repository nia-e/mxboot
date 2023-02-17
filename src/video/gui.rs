use anyhow::Result;
use embedded_graphics::prelude::*;
use lvgl::{UI, LvError, style::Style, State, Color, Widget, Part};
use std::{time::Duration, thread::sleep};
#[cfg(not(target_arch = "aarch64"))]
use std::mem::transmute;
#[cfg(not(target_arch = "aarch64"))]
use embedded_graphics_simulator::{Window, SimulatorEvent};
#[cfg(not(target_arch = "aarch64"))]
use embedded_graphics::pixelcolor::Rgb565;

pub fn load_gui<D: DrawTarget + OriginDimensions, T>(display: D, mut window: Option<T>) -> Result<(), LvError>
where <D as DrawTarget>::Color: From<Color> {
    let mut ui = UI::init()?;
    ui.disp_drv_register(display)?;
    let mut screen = ui.scr_act()?;
    let mut screen_style = Style::default();
    screen_style.set_bg_color(State::DEFAULT, Color::from_rgb((255, 255, 255)));
    screen_style.set_radius(State::DEFAULT, 0);
    screen.add_style(Part::Main, screen_style)?;
    'running: loop {
        ui.task_handler();
        #[cfg(not(target_arch = "aarch64"))]
        unsafe {
            let mut w: &mut Window = transmute(window.as_mut().unwrap());
            w.update::<Rgb565>(transmute(ui.get_display_ref().unwrap()));
            for event in w.events() {
                match event {
                    SimulatorEvent::Quit => break 'running,
                    _ => {}
                }
            }
        }
        sleep(Duration::from_millis(10));
    }
    Ok(())
}