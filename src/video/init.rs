#![cfg(target_arch = "aarch64")]

use anyhow::Result;
use framebuffer::Framebuffer;
use super::fb::DrawableFramebuffer;
use super::gui::load_gui;

pub fn init_fb(fb: Framebuffer) -> Result<()> {
    let mut fb = DrawableFramebuffer::new(fb)?;
    match load_gui(fb, None::<u8>) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
    Ok(())
}