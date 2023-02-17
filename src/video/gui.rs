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

pub(crate) fn load_gui<D: DrawTarget + OriginDimensions, T>(display: D, mut window: Option<T>) -> Result<(), LvError>
where <D as DrawTarget>::Color: From<Color> {
    let mut ui = UI::init()?;
    ui.disp_drv_register(display)?;
    let mut screen = ui.scr_act()?;

    // Styling, TODO: implement unl0kr's theme.
    let mut screen_style = Style::default();
    screen_style.set_bg_color(State::DEFAULT, Color::from_rgb((255, 255, 255)));
    screen_style.set_radius(State::DEFAULT, 0);
    screen.add_style(Part::Main, screen_style)?;

    'running: loop {
        ui.task_handler();
        // Not proud of all the transmutes, but it's the only way to make this
        // generic over simulated vs real displays without extensive code
        // duplication which would probably be worse.
        #[cfg(not(target_arch = "aarch64"))]
        unsafe {
            let w: &mut Window = transmute(window.as_mut().unwrap());
            w.update::<Rgb565>(transmute(ui.get_display_ref().unwrap()));
            for event in w.events() {
                if event == SimulatorEvent::Quit { break 'running }
            }
        }
        sleep(Duration::from_millis(16));
    }
    Ok(())
}