use anyhow::Result;
use framebuffer::{Framebuffer, KdMode};

use super::fb::DrawableFramebuffer;

pub fn init_fb(fb: Framebuffer) -> Result<()> {
    let fb = DrawableFramebuffer::new(fb)?;
    Ok(())
}
