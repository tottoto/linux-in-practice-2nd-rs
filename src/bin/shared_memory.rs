const PAGE_SIZE: usize = 4 * 1024;

fn main() {
    let data: u16 = 1000;

    println!("data value before spawning child process: {data}");
    let shared_memory = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            PAGE_SIZE,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_ANONYMOUS | libc::MAP_SHARED,
            -1,
            0,
        )
    };

    unsafe {
        *(shared_memory as *mut u16) = data;
    }

    let pid: u32 = unsafe { libc::fork() }
        .try_into()
        .expect("failed to fork process");

    if pid == 0 {
        unsafe {
            *(shared_memory as *mut u16) *= 2;
        }
        std::process::exit(0);
    }

    let mut status = 0;
    let _ = unsafe { libc::waitpid(pid as i32, &mut status, 0) };
    assert!(
        libc::WIFEXITED(status),
        "child process `{pid}` did not terminate normally"
    );

    let data = unsafe { *(shared_memory as *const u16) };
    println!("data value after finishing child process: {data}");

    if unsafe { libc::munmap(shared_memory, PAGE_SIZE) } == -1 {
        panic!("failed to call munmap()");
    }
}
