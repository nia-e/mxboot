#![cfg(target_arch = "aarch64")]

use super::fb::DrawableFramebuffer;
use super::gui::load_gui;
use anyhow::Result;
use framebuffer::Framebuffer;

pub fn init_fb(fb: Framebuffer) -> Result<()> {
    let mut fb = DrawableFramebuffer::new(fb)?;
    // Code Crimes(tm) have been done here to reduce executable size on aarch64
    // so we don't pull in the embedded_graphics_simulator crate. The None is
    // entirely unused.
    unsafe {
        match load_gui(fb, None::<u8>) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
    Ok(())
}
