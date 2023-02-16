use anyhow::Result;
use framebuffer::Framebuffer;
use std::process::Command;
#[cfg(target_arch = "aarch64")]
use syscalls::{
    aarch64::Sysno::{reboot, sync},
    syscall, syscall_args,
};

pub mod video;

#[cfg(target_arch = "aarch64")]
static RB_AUTOBOOT: usize = 0x1234567; // Reboot syscall magic

fn main() -> Result<()> {
    match Framebuffer::new("/dev/FIXME") {
        Ok(fb) => match video::init::init_fb(fb) {
            Ok(_) => (),
            Err(e) => eprintln!("{e}"),
        },
        Err(e) => {
            eprintln!("{e}\nmxboot failed to initialize framebuffer. Booting previously set OS...")
        }
    }

    // Set boot as valid; if this fails, no clear behavior except crashing
    let _ = Command::new("qbootctl").arg("-m").output()?;
    // Get current slot
    let slot = Command::new("qbootctl").arg("-c").output()?.stdout;
    let new_slot = if "a".as_bytes().eq(&slot) { "b" } else { "a" };
    // Switch slot
    let _ = Command::new("qbootctl").arg("-s").arg(new_slot).output()?;

    #[cfg(target_arch = "aarch64")]
    unsafe {
        match syscall(sync, &syscall_args!()) {
            Ok(_) => (),
            Err(e) => eprintln!("{e}\nmxboot failed to sync(2). Attempting reboot anyways"),
        }

        match syscall(reboot, &syscall_args!(RB_AUTOBOOT)) {
            Ok(_) => (),
            Err(e) => panic!("{e}\nmxboot failed to reboot system. Exiting...")
        }
    }
    Ok(())
}
