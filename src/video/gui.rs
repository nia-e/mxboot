#[cfg(not(target_arch = "aarch64"))]
use embedded_graphics::pixelcolor::Rgb565;
#[cfg(not(target_arch = "aarch64"))]
use embedded_graphics_simulator::{SimulatorEvent, Window};
#[cfg(not(target_arch = "aarch64"))]
use std::mem::transmute;

use super::{contains::get_obj_at_pt, term_ui, theme::MxTheme};
use cstr_core::CString;
use embedded_graphics::prelude::*;
use lvgl::{widgets, Align, Color, Event, LvError, Part, Widget, UI};
use std::{sync::mpsc::channel, time::Duration};

/// Possible screens to which the UI can navigate. `Exit` represents quitting
/// mxboot.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NavLocation {
    Home,
    Terminal,
    Exit,
}

/// A GUI event to be processed in the main event loop.
#[derive(Debug, Clone, Copy)]
pub enum GuiEvent {
    Navigate(NavLocation)
}

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
    let theme = MxTheme::pmos_dark();
    let mut screen = ui.scr_act()?;
    screen.add_style(Part::Main, theme.style_window())?;

    let mut button = widgets::Btn::new(&mut screen)?;
    button.set_align(&mut screen, Align::InTopMid, 0, 0)?;
    button.set_size(200, 100)?;
    button.add_style(Part::Main, theme.style_button())?;

    let mut label = widgets::Label::new(&mut button)?;
    label.set_text(CString::new("Terminal").unwrap().as_c_str())?;
    label.add_style(Part::Main, theme.style_label())?;

    let (tx, rx) = channel::<GuiEvent>();
    let mut current_scr = NavLocation::Home;

    /*
    let mut kb = widgets::Keyboard::new(&mut screen)?;
    kb.set_align(&mut screen, Align::InBottomMid, 0, 0)?;
    kb.set_size(1080, 540).unwrap();
    kb.add_style(Part::Main, theme.style_keyboard()).unwrap();
    kb.set_cursor_manage(true)?;
    */

    button.on_event(|_, event| {
        if let lvgl::Event::Clicked = event {
            println!("Clicked!");
            match tx.send(GuiEvent::Navigate(NavLocation::Terminal)) {
                Ok(_) => (),
                Err(e) => eprintln!("{e}")
            }
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
                println!("{:?}", event);
                match event {
                    SimulatorEvent::MouseButtonUp {
                        mouse_btn: _,
                        point,
                    } => {
                        if let Some(obj) = get_obj_at_pt(&screen, &point) {
                            println!("Did")
                        }
                    }
                    SimulatorEvent::Quit => match tx.send(GuiEvent::Navigate(NavLocation::Exit)) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("{e}");
                            break 'running
                        }
                    }
                    _ => {}
                }
            }
        }
        while let Ok(event) = rx.try_recv() {
            match event {
                GuiEvent::Navigate(route) => {
                    println!("Navigating to {:?}", route);
                    match route {
                        NavLocation::Terminal => {
                            let mut term_screen = term_ui::term_ui(&theme, &tx)?;
                            ui.load_scr(&mut term_screen)?;
                            current_scr = NavLocation::Terminal
                        }
                        NavLocation::Home => {
                            ui.load_scr(&mut screen)?;
                            current_scr = NavLocation::Home
                        },
                        NavLocation::Exit => break 'running,
                    }
                }
            }
        }
        ui.tick_inc(Duration::from_millis(16));
    }
    Ok(())
}
