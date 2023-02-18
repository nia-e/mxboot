#[cfg(not(target_arch = "aarch64"))]
use embedded_graphics::pixelcolor::Rgb565;
#[cfg(not(target_arch = "aarch64"))]
use embedded_graphics_simulator::{SimulatorEvent, Window};
#[cfg(not(target_arch = "aarch64"))]
use std::mem::transmute;

use anyhow::Result;
use cstr_core::CString;
use embedded_graphics::prelude::*;
use lvgl::{style::Style, widgets, Align, Color, Event, LvError, Part, State, Widget, UI};
use std::{thread::sleep, time::Duration};

pub(crate) unsafe fn load_gui<D: DrawTarget + OriginDimensions, T>(
    display: D,
    mut window: Option<T>,
) -> Result<(), LvError>
where
    <D as DrawTarget>::Color: From<Color>,
{
    let mut ui = UI::init()?;
    ui.disp_drv_register(display)?;
    let mut screen = ui.scr_act()?;

    // Styling, TODO: implement unl0kr's theme.
    let mut screen_style = Style::default();
    screen_style.set_bg_color(State::DEFAULT, Color::from_rgb((255, 255, 255)));
    screen_style.set_radius(State::DEFAULT, 0);
    screen.add_style(Part::Main, screen_style)?;

    let mut button = widgets::Btn::new(&mut screen)?;
    button.set_align(&mut screen, Align::Center, 0, 0)?;
    button.set_size(200, 100)?;
    let mut label = widgets::Label::new(&mut button)?;
    label.set_text(CString::new("Click me!").unwrap().as_c_str())?;

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
                    SimulatorEvent::MouseButtonUp { .. } => {
                        ui.event_send(&mut button, Event::Clicked)?
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
