#![cfg(target_arch = "aarch64")]

use anyhow::Result;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb888, prelude::*};
use framebuffer::{Framebuffer, KdMode};
use std::num::TryFromIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DrawableFbError {
    #[error("bogus panel size")]
    BogusPanel(TryFromIntError),
}

/// A wrapper around framebuffer's Framebuffer type, `impl`'ing the trait
/// embedded-graphics (and therefore LVGL) uses to talk to displays.
/// self.flush() must be called to write the frame to the framebuffer.
pub struct DrawableFramebuffer {
    pub frame: Vec<u8>,
    fb_dev: Framebuffer,
}

impl DrawableFramebuffer {
    /// Instantiates a DrawableFramebuffer from a given framebuffer device.
    pub fn new(fb_dev: Framebuffer) -> Result<DrawableFramebuffer> {
        let h = fb_dev.var_screen_info.yres;
        let line_length = fb_dev.fix_screen_info.line_length;
        let frame = vec![0u8; (line_length * h) as usize];

        match Framebuffer::set_kd_mode(KdMode::Graphics) {
            Ok(_) => (),
            Err(e) => return Err(e.into()),
        }

        Ok(DrawableFramebuffer { frame, fb_dev })
    }

    /// Writes self.frame into the framebuffer device proper.
    pub fn flush(&mut self) {
        self.fb_dev.write_frame(&self.frame)
    }
}

impl DrawTarget for DrawableFramebuffer {
    type Color = Rgb888;

    type Error = DrawableFbError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        // None of these should ever fail, realistically, but there may be
        // errors while reading so it's worth at least gracefully exiting.
        let w: i32 = match self.fb_dev.var_screen_info.width.try_into() {
            Ok(w) => w,
            Err(e) => return Err(Self::Error::BogusPanel(e)),
        };
        let h: i32 = match self.fb_dev.var_screen_info.height.try_into() {
            Ok(h) => h,
            Err(e) => return Err(Self::Error::BogusPanel(e)),
        };
        let line_length: i32 = match self.fb_dev.fix_screen_info.line_length.try_into() {
            Ok(l) => l,
            Err(e) => return Err(Self::Error::BogusPanel(e)),
        };
        let bytespp: i32 = match (self.fb_dev.var_screen_info.bits_per_pixel / 8).try_into() {
            Ok(b) => b,
            Err(e) => return Err(Self::Error::BogusPanel(e)),
        }; // Should be 3 but better to draw *something approximately ok*
           // to than fail entirely otherwise.

        for Pixel(coord, color) in pixels.into_iter() {
            if coord.x < w && coord.x >= 0 && coord.y < h && coord.y >= 0 {
                let index: u32 = match (coord.x * bytespp + coord.y * line_length).try_into() {
                    Ok(i) => i,
                    Err(e) => return Err(Self::Error::BogusPanel(e)),
                };
                self.frame[index as usize] = color.r();
                self.frame[(index + 1) as usize] = color.g();
                self.frame[(index + 2) as usize] = color.b();
            }
        }
        Ok(())
    }
}

impl OriginDimensions for DrawableFramebuffer {
    fn size(&self) -> Size {
        Size::new(
            self.fb_dev.var_screen_info.width,
            self.fb_dev.var_screen_info.height,
        )
    }
}

impl Drop for DrawableFramebuffer {
    // Ensure that we don't stay in graphics mode after dropping the handle
    fn drop(&mut self) {
        self.flush();
        match Framebuffer::set_kd_mode(KdMode::Text) {
            Ok(_) => (),
            Err(e) => eprintln!("{e}"),
        }
    }
}
