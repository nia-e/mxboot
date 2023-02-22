//#![cfg(not(target_arch = "aarch64"))]

use embedded_graphics::prelude::Point;
use lvgl::{LvError, Widget};

/// Checks if a given `Point` is inside of the widget.
pub fn contains(object: &impl Widget, point: &Point) -> Result<bool, LvError> {
    // TODO: upstream this function into LVGL
    let coords = unsafe { &object.raw()?.as_ref().coords };
    let (x1, y1, x2, y2) = (
        coords.x1 as i32,
        coords.y1 as i32,
        coords.x2 as i32,
        coords.y2 as i32,
    );
    if x1 < point.x && x2 > point.x && y1 < point.y && y2 > point.y {
        Ok(true)
    } else {
        Ok(false)
    }
}
