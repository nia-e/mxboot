use super::{
    gui::{GuiEvent, NavLocation},
    theme::MxTheme,
};
use cstr_core::CString;
use lvgl::{widgets, Screen, Align, LvError, NativeObject, Part, Widget};
use lvgl_sys::{lv_obj_t};
use std::sync::mpsc::Sender;

/// Loads the terminal emulator UI on the display.
pub fn term_ui(theme: &MxTheme, tx: &Sender<GuiEvent>) -> Result<Screen, LvError> {
    // Need to create a new screen for the terminal UI
    let mut screen = Screen::default();
    let mut screen_style = theme.style_window();
    screen.add_style(Part::Main, &mut screen_style)?;

    let mut kb = widgets::Keyboard::create(&mut screen)?;
    kb.set_align(Align::BottomMid, 0, 0)?;
    kb.set_size(1080, 540).unwrap();
    let mut kb_style = theme.style_keyboard();
    kb.add_style(Part::Main, &mut kb_style).unwrap();
    //kb.set_cursor_manage(true)?;

    let mut ta = widgets::Textarea::create(&mut screen)?;
    ta.set_align(Align::TopMid, 0, 0)?;
    ta.set_size(1080, 1620)?;
    ta.set_text(CString::new("").unwrap().as_c_str())?;
    let mut ta_style = theme.style_text_area();
    ta.add_style(Part::Main, &mut ta_style)?;

    // lv_keyboard_set_textarea not implemented yet in the LVGL crate
    unsafe {
        lvgl_sys::lv_keyboard_set_textarea(
            kb.raw()?.as_mut() as *mut lv_obj_t,
            ta.raw()?.as_mut() as *mut lv_obj_t,
        )
    }

    Ok(screen)
}
