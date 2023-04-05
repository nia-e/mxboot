use super::{
    gui::{GuiEvent, NavLocation},
    theme::MxTheme,
};
use cstr_core::CString;
use lvgl::{widgets, Align, LvError, NativeObject, Obj, Part, Widget};
use lvgl_sys::{_lv_obj_t, lv_obj_create};
use std::{ptr, sync::mpsc::Sender};

/// Loads the terminal emulator UI on the display.
///
/// # Safety
///
/// If the target is `aarch64`, `window` must be an object of type
/// `embedded_graphics_simulator::Window`. This is not enforced by the
/// typesystem as that would require pulling in `embedded_graphics_simulator`
pub unsafe fn term_ui(theme: &MxTheme, tx: &Sender<GuiEvent>) -> Result<Obj, LvError> {
    // Need to create a new screen for the terminal UI
    let mut screen =
        Obj::from_raw(ptr::NonNull::new(lv_obj_create(ptr::null_mut(), ptr::null_mut())).unwrap());
    screen.add_style(Part::Main, theme.style_window())?;

    let mut kb = widgets::Keyboard::create(&mut screen)?;
    kb.set_align(Align::BottomMid, 0, 0)?;
    kb.set_size(1080, 540).unwrap();
    kb.add_style(Part::Main, theme.style_keyboard()).unwrap();
    kb.set_cursor_manage(true)?;

    let mut ta = widgets::Textarea::new(&mut screen)?;
    ta.set_align(&mut screen, Align::InTopMid, 0, 0)?;
    ta.set_size(1080, 1620)?;
    ta.set_text(CString::new("").unwrap().as_c_str())?;
    ta.add_style(Part::Main, theme.style_text_area())?;

    // lv_keyboard_set_textarea not implemented yet in the LVGL crate
    unsafe {
        lvgl_sys::lv_keyboard_set_textarea(
            kb.raw()?.as_mut() as *mut _lv_obj_t,
            ta.raw()?.as_mut() as *mut _lv_obj_t,
        )
    }

    Ok(screen)
}
