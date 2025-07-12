const ALLOC_SIZE: usize = 100 * 1024 * 1024;
const ACCESS_UNIT: usize = 10 * 1024 * 1024;
const PAGE_SIZE: usize = 4 * 1024;

fn show_message(msg: &str) {
    const TIME_FORMAT: &std::ffi::CStr = c"%H:%M:%S";
    const TIME_SIZE: usize = TIME_FORMAT.to_bytes_with_nul().len();

    let now = unsafe {
        let now = libc::localtime(&libc::time(std::ptr::null_mut()));
        let buf = libc::malloc(TIME_SIZE);
        let _ = libc::strftime(buf as *mut _, TIME_SIZE, TIME_FORMAT.as_ptr(), now);
        let formatted = std::ffi::CStr::from_ptr(buf as *const _)
            .to_str()
            .unwrap_or_else(|e| panic!("timestamp contains invalid UTF-8 data: {e}"))
            .to_owned();
        libc::free(buf);
        formatted
    };

    println!("{now}: {msg}");
}

fn wait_enter(buf: &mut String) {
    std::io::stdin().read_line(buf).unwrap();
    buf.clear();
}

fn main() {
    show_message("Before allocating new memory. Press ENTER to allocate new 100MiB memory.");

    let mut buf = String::new();
    wait_enter(&mut buf);

    let memregion = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            ALLOC_SIZE,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        )
    };

    if memregion == libc::MAP_FAILED {
        panic!("failed to call mmap()");
    }

    show_message("Allocated new memory. Press ENTER to access the allocated memory by 10MiB/1s.");
    wait_enter(&mut buf);

    for i in (0..ALLOC_SIZE).step_by(PAGE_SIZE) {
        unsafe {
            *(memregion.wrapping_add(i) as *mut std::ffi::c_int) = 0;
        }
        if i % ACCESS_UNIT == 0 && i != 0 {
            show_message(&format!("Acessed {} MiB memory", i / (1024 * 1024)));
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    show_message("Finished to access to the all allocated memory. Press ENTER to exit.");
    wait_enter(&mut buf);

    if unsafe { libc::munmap(memregion, ALLOC_SIZE) } == -1 {
        panic!("failed to call munmap()");
    }
}
