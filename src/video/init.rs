#![cfg(target_arch = "aarch64")]

use super::gui::load_gui;
use super::{HOR_RES, VER_RES};
use anyhow::Result;
use framebuffer::Framebuffer;
use lvgl::DrawBuffer;

/// Initialize the display, given a framebuffer device.
pub fn init_fb() -> Result<()> {
    let buf = DrawBuffer::<{(HOR_RES * VER_RES / 10) as usize}>::default();
    let mut fb = lvgl::lv_drv_disp_fbdev!(buf, HOR_RES, VER_RES);
    match load_gui(fb) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
    Ok(())
}
