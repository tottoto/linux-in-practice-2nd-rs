fn main() {
    let ret: u32 = unsafe { libc::fork() }
        .try_into()
        .expect("failed to fork process");
    if ret == 0 {
        let pid = unsafe { libc::getpid() };
        let ppid = unsafe { libc::getppid() };
        println!("child process: pid={pid}, parent process pid={ppid}");
        let _ = unsafe {
            libc::execve(
                c"/bin/echo".as_ptr(),
                [
                    c"echo".as_ptr(),
                    std::ffi::CString::new(format!("hello from pid={pid}"))
                        .unwrap()
                        .as_ptr(),
                    std::ptr::null(),
                ]
                .as_ptr(),
                std::ptr::null(),
            )
        };
    } else if ret > 0 {
        let pid = unsafe { libc::getpid() };
        println!("parent process: pid={pid}, child process pid={ret}");
    }
}
