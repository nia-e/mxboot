use anyhow::Result;
#[cfg(target_arch = "aarch64")]
use framebuffer::Framebuffer;
#[cfg(target_arch = "aarch64")]
use std::process::Command;
#[cfg(target_arch = "aarch64")]
use syscalls::{
    aarch64::Sysno::{reboot, sync},
    syscall, syscall_args,
};

mod video;

const HOR_RES: u32 = 480;
const VER_RES: u32 = 480;

/// Linux `reboot(2)` syscall magic for system restart.
#[cfg(target_arch = "aarch64")]
const RB_AUTOBOOT: usize = 0x1234567;

fn main() -> Result<()> {
    #[cfg(target_arch = "aarch64")]
    match video::init::init_fb() {
        Ok(_) => (),
        Err(e) => eprintln!("{e}"),
    }

    #[cfg(not(target_arch = "aarch64"))]
    match video::mock_init::init_disp() {
        Ok(_) => (),
        Err(e) => eprintln!("{e}"),
    }

    // Set boot as valid; if this fails, it's ok to carry on
    #[cfg(target_arch = "aarch64")]
    if let Err(e) = Command::new("qbootctl").arg("-m").output() {
        eprintln!("{e}");
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        match syscall(sync, &syscall_args!()) {
            Ok(_) => (),
            // Not *super* sure it's ok to reboot but whatever
            Err(e) => eprintln!("{e}\nmxboot failed to sync(2). Attempting reboot anyways"),
        }

        match syscall(reboot, &syscall_args!(RB_AUTOBOOT)) {
            Ok(_) => (),
            Err(e) => panic!("{e}\nmxboot failed to reboot(2). Exiting..."),
        }
    }
    Ok(())
}
