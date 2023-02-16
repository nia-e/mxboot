#![cfg(target_arch = "aarch64")]

use anyhow::Result;
use framebuffer::Framebuffer;
use super::fb::DrawableFramebuffer;
use super::gui::load_gui;

pub fn init_fb(fb: Framebuffer) -> Result<()> {
    let mut fb = DrawableFramebuffer::new(fb)?;
    load_gui(fb)?;
    Ok(())
}