const LOCK_FILE: &std::ffi::CStr = c"lock";
const COUNTER_FILE: &str = "count";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lock_fd = unsafe { libc::open(LOCK_FILE.as_ptr(), libc::O_CREAT) };

    loop {
        if unsafe { libc::flock(lock_fd, libc::LOCK_EX) } == 0 {
            break;
        }
    }

    let value: u32 = std::fs::read_to_string(COUNTER_FILE)?.trim().parse()?;
    std::fs::write(COUNTER_FILE, format!("{}\n", value + 1).as_bytes())?;

    if unsafe { libc::flock(lock_fd, libc::LOCK_UN) } != 0 {
        panic!("failed to unlock");
    }

    Ok(())
}
