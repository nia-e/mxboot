#![cfg(not(target_arch = "aarch64"))]

use super::gui::load_gui;
use crate::{HOR_RES, VER_RES};
use anyhow::Result;
use lvgl::DrawBuffer;

/// Initialize the display.
pub fn init_disp() -> Result<()> {
    let buf = DrawBuffer::<{(HOR_RES * VER_RES / 10) as usize}>::default();
    let mut fb = lvgl::lv_drv_disp_sdl!(buf, HOR_RES, VER_RES).unwrap();
    match load_gui(fb) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
    Ok(())
}