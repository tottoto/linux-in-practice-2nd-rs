const ALLOC_SIZE: usize = 100 * 1024 * 1024;
const PAGE_SIZE: usize = 4 * 1024;

fn access(data: *mut std::ffi::c_void, size: usize, step: usize) {
    for i in (0..size).step_by(step) {
        unsafe {
            *(data.wrapping_add(i) as *mut std::ffi::c_int) = 0;
        }
    }
}

fn show_meminfo(msg: &str, process: &str) {
    println!("{msg}");
    println!("free output:");
    let _ = std::process::Command::new("free")
        .stdout(std::io::stdout())
        .output()
        .expect("failed to execute free command");
    let pid = unsafe { libc::getpid() }.to_string();
    println!("memory information of {process}");
    let _ = std::process::Command::new("ps")
        .args(["-orss,maj_flt,min_flt", &pid])
        .stdout(std::io::stdout())
        .output()
        .expect("failed to execute free command");
}

fn main() {
    let data = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            ALLOC_SIZE,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        )
    };

    access(data, ALLOC_SIZE, PAGE_SIZE);

    show_meminfo("*** before spawning child process ***", "parent process");

    let pid: u32 = unsafe { libc::fork() }
        .try_into()
        .expect("failed to fork process");
    if pid == 0 {
        show_meminfo("*** just after spawning child process ***", "child process");
        access(data, ALLOC_SIZE, PAGE_SIZE);
        show_meminfo(
            "*** after child process accessing to the memory",
            "child process",
        );
        return;
    }

    let mut status = 0;
    let _ = unsafe { libc::waitpid(pid as i32, &mut status, 0) };
    assert!(
        libc::WIFEXITED(status),
        "child process `{pid}` did not terminate normally"
    );
}
