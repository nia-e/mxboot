#[cfg(target_arch = "aarch64")]
pub mod fb;
#[cfg(target_arch = "aarch64")]
pub mod init;

#[cfg(not(target_arch = "aarch64"))]
pub mod mock_init;
//#[cfg(not(target_arch = "aarch64"))]
pub mod contains;

pub mod gui;
pub mod term_ui;
pub mod theme;
