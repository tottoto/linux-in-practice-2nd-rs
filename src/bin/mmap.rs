const ALLOC_SIZE: usize = 1024 * 1024 * 1024; // 1GiB

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pid = unsafe { libc::getpid() };

    let mut cat_maps = {
        let mut command = std::process::Command::new("cat");
        command.arg(format!("/proc/{pid}/maps"));
        command.stdout(std::io::stdout());
        command
    };

    println!("*** Memory map before allocating new memory ***");
    let _ = cat_maps.output()?;

    let new_memory = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            ALLOC_SIZE,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        )
    };

    if new_memory == libc::MAP_FAILED {
        panic!("failed to call mmap()");
    }

    println!();
    println!("*** Allocated memory: address = {new_memory:#p}; size = {ALLOC_SIZE:#x} ***");

    println!();
    println!("*** Memory map after allocating new memory ***");
    let _ = cat_maps.output()?;

    if unsafe { libc::munmap(new_memory, ALLOC_SIZE) } == -1 {
        panic!("failed to call munmap()");
    }

    Ok(())
}
