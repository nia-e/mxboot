#[cfg(target_arch = "aarch64")]
pub mod fb;
#[cfg(target_arch = "aarch64")]
pub mod init;
#[cfg(not(target_arch = "aarch64"))]
pub mod mock_init;
pub mod gui;