#[cfg(not(target_arch = "aarch64"))]
use embedded_graphics::pixelcolor::Rgb565;
#[cfg(not(target_arch = "aarch64"))]
use embedded_graphics_simulator::{SimulatorEvent, Window};
#[cfg(not(target_arch = "aarch64"))]
use std::mem::transmute;

use super::theme::MxTheme;
use cstr_core::CString;
use embedded_graphics::prelude::*;
use lvgl::{widgets, Align, Color, Event, LvError, NativeObject, Obj, Part, Widget, UI};
use lvgl_sys::{_lv_obj_t, lv_obj_create};
use std::{ptr, thread::sleep, time::Duration};

/// Loads the terminal emulator UI on the display.
///
/// # Safety
///
/// If the target is `aarch64`, `window` must be an object of type
/// `embedded_graphics_simulator::Window`. This is not enforced by the
/// typesystem as that would require pulling in `embedded_graphics_simulator`
pub unsafe fn term_ui<D: DrawTarget + OriginDimensions, T>(
    ui: &mut UI<D, <D as DrawTarget>::Color>,
    theme: &MxTheme,
    window: &mut Option<T>,
) -> Result<(), LvError>
where
    <D as DrawTarget>::Color: From<Color>,
{
    println!("In");
    // Need to create a new screen for the terminal UI
    let mut screen =
        Obj::from_raw(ptr::NonNull::new(lv_obj_create(ptr::null_mut(), ptr::null_mut())).unwrap());
    screen.add_style(Part::Main, theme.style_window())?;

    let mut kb = widgets::Keyboard::new(&mut screen)?;
    kb.set_align(&mut screen, Align::InBottomMid, 0, 0)?;
    kb.set_size(1080, 540).unwrap();
    kb.add_style(Part::Main, theme.style_keyboard()).unwrap();
    kb.set_cursor_manage(true)?;

    let mut ta = widgets::Textarea::new(&mut screen)?;
    ta.set_align(&mut screen, Align::InTopMid, 0, 0)?;
    ta.set_size(1080, 1620)?;
    ta.set_text(CString::new("").unwrap().as_c_str())?;
    ta.add_style(Part::Main, theme.style_text_area())?;

    ui.load_scr(&mut screen)?;

    // lv_keyboard_set_textarea not implemented yet in the LVGL crate
    unsafe {
        lvgl_sys::lv_keyboard_set_textarea(
            kb.raw()?.as_mut() as *mut _lv_obj_t,
            ta.raw()?.as_mut() as *mut _lv_obj_t,
        )
    }

    'running: loop {
        ui.task_handler();

        // See gui.rs:59
        #[cfg(not(target_arch = "aarch64"))]
        unsafe {
            let w: &mut Window = transmute(window.as_mut().unwrap());
            w.update::<Rgb565>(transmute(ui.get_display_ref().unwrap()));
            for event in w.events() {
                match event {
                    SimulatorEvent::MouseButtonUp {
                        mouse_btn: _,
                        point: _,
                    } => {
                        //if contains(&button, &point)? {
                        //    ui.event_send(&mut button, Event::Clicked)?
                        //}
                    }
                    // Ideally should quit window entirely but idc honestly
                    SimulatorEvent::Quit => break 'running,
                    _ => {}
                }
            }
        }
        sleep(Duration::from_millis(16));
    }
    Ok(())
}
