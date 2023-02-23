//#![cfg(not(target_arch = "aarch64"))]

use embedded_graphics::prelude::Point;
use lvgl::{LvError, NativeObject, Obj};
use lvgl_sys::{_lv_obj_t, lv_obj_get_child};
use std::ptr;

/// Checks if a given `Point` is inside of the object. Used internally by
/// `get_obj_at_pt()`.
/// 
/// # Safety
/// 
/// `object` must point to an initialized instance of `_lv_obj_t`.
unsafe fn contains(object: *const _lv_obj_t, point: &Point) -> Result<bool, LvError> {
    let coords = (*object).coords;
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

/// Recursively searches down the widget tree for the lowest object below the
/// `Point`. Used internally by `get_obj_at_pt()`.
/// 
/// # Safety
/// 
/// `parent` must point to an initialized instance of `_lv_obj_t`.
unsafe fn rec_get_frontmost(
    parent: *mut _lv_obj_t,
    point: &Point,
) -> Result<*mut _lv_obj_t, LvError> {
    let mut current = ptr::null_mut();
    unsafe {
        'search: loop {
            match lv_obj_get_child(parent, current) as usize {
                0 => break 'search,
                p => {
                    current = p as *mut _lv_obj_t;
                    if contains(current, point)? {
                        return rec_get_frontmost(current, point);
                    }
                }
            }
        }
    }
    Ok(parent)
}

/// Recursively finds the frontmost object at a given `Point`.
pub fn get_obj_at_pt<'a>(screen: &'a Obj, point: &Point) -> Option<&'a mut _lv_obj_t> {
    unsafe {
        let scr_raw: &'a mut _lv_obj_t = screen.raw().ok()?.as_mut();
        // TODO: Implement IntoIter for getting children of objects in upstream
        let mut current: &'a mut _lv_obj_t = lv_obj_get_child(scr_raw, ptr::null_mut()).as_mut()?;
        'search: loop {
            if contains(current, point).ok()? {
                return match rec_get_frontmost(current, point) {
                    Ok(ptr) => Some(&mut *ptr),
                    Err(e) => {
                        eprintln!("{:?}", e);
                        None
                    }
                };
            } else {
                match lv_obj_get_child(scr_raw, current) as usize {
                    // lv_obj_get_child will return a null ptr if exhausted
                    0 => break 'search,
                    child => current = &mut *(child as *mut _lv_obj_t),
                }
            }
        }
    }
    None
}
