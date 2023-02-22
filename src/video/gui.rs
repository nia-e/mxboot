#[cfg(not(target_arch = "aarch64"))]
use embedded_graphics::pixelcolor::Rgb565;
#[cfg(not(target_arch = "aarch64"))]
use embedded_graphics_simulator::{SimulatorEvent, Window};
#[cfg(not(target_arch = "aarch64"))]
use std::mem::transmute;

use anyhow::Result;
use cstr_core::CString;
use embedded_graphics::prelude::*;
use lvgl::{widgets, Align, Color, Event, LvError, Part, Widget, UI};
use std::{thread::sleep, time::Duration};
use super::{theme::MxTheme, contains::contains};

/// Loads the actual GUI on the display.
///
/// # Safety
///
/// If the target is `aarch64`, `window` must be an object of type
/// `embedded_graphics_simulator::Window`. This is not enforced by the
/// typesystem as that would require pulling in `embedded_graphics_simulator`
/// on `aarch64` builds, increasing executable size.
pub unsafe fn load_gui<D: DrawTarget + OriginDimensions, T>(
    display: D,
    mut window: Option<T>,
) -> Result<(), LvError>
where
    <D as DrawTarget>::Color: From<Color>,
{
    let mut ui = UI::init()?;
    ui.disp_drv_register(display)?;
    let mut screen = ui.scr_act()?;
    let theme = MxTheme::pmos_dark();

    // Styling, TODO: finish implementing unl0kr's theme.
    screen.add_style(Part::Main, theme.style_window())?;

    let mut button = widgets::Btn::new(&mut screen)?;
    button.set_align(&mut screen, Align::InTopMid, 0, 0)?;
    button.set_size(200, 100)?;
    button.add_style(Part::Main, theme.style_button())?;
    let mut label = widgets::Label::new(&mut button)?;
    label.set_text(CString::new("Click me!").unwrap().as_c_str())?;
    label.add_style(Part::Main, theme.style_label())?;

    button.on_event(|mut btn, event| {
        if let lvgl::Event::Clicked = event {
            println!("Clicked");
            btn.toggle().unwrap();
        }
    })?;

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
                match event {
                    SimulatorEvent::MouseButtonUp { mouse_btn: _, point } => {
                        if contains(&button, &point)? {
                            ui.event_send(&mut button, Event::Clicked)?
                        }
                    }
                    SimulatorEvent::Quit => break 'running,
                    _ => {}
                }
            }
        }
        sleep(Duration::from_millis(16));
    }
    Ok(())
}
