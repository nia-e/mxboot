use anyhow::Result;
#[cfg(not(target_arch = "aarch64"))]
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
#[cfg(not(target_arch = "aarch64"))]
use embedded_graphics_simulator::{SimulatorEvent, Window};
use lvgl::{style::Style, Color, LvError, Part, State, Widget, UI};
#[cfg(not(target_arch = "aarch64"))]
use std::mem::transmute;
use std::{thread::sleep, time::Duration};

pub(crate) fn load_gui<D: DrawTarget + OriginDimensions, T>(
    display: D,
    mut window: Option<T>,
) -> Result<(), LvError>
where
    <D as DrawTarget>::Color: From<Color>,
{
    let mut ui = UI::init()?;
    ui.disp_drv_register(display)?;
    let screen = ui.scr_act()?;

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
                if event == SimulatorEvent::Quit {
                    break 'running;
                }
            }
        }
        sleep(Duration::from_millis(16));
    }
    Ok(())
}
