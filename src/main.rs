use anyhow::Result;
use framebuffer::Framebuffer;
use std::process::Command;
use linux::syscall::{sync, reboot};

pub mod fb;

fn main() -> Result<()> {
    match Framebuffer::new("/dev/FIXME") {
        Ok(fb) => fb::init_fb(fb),
        Err(e) => eprint!("{e}\nmxboot failed to initialize framebuffer. Booting previously set OS...")
    }

    // Set boot as valid; if this fails, no clear behavior except crashing
    let _ = Command::new("qbootctl").arg("-m").output()?;
    // Get current slot
    let slot = Command::new("qbootctl").arg("-c").output()?.stdout;
    let new_slot = if "a".as_bytes().eq(&slot) { "b" } else { "a" };
    // Switch slot
    let _ = Command::new("qbootctl").arg("-s").arg(new_slot).output()?;

    if sync() != 0 {
        eprintln!("WARNING: sync(2) failed. Attempting reboot anyways")
    }

    if reboot(0x1234567, "") == 0 {
        Ok(())
    }
    else {
        panic!("mxboot failed to reboot system. Exiting...")
    }
}
