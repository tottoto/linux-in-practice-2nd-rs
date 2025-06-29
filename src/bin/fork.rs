fn main() {
    let ret: u32 = unsafe { libc::fork() }
        .try_into()
        .expect("failed to fork process");
    if ret == 0 {
        let pid = unsafe { libc::getpid() };
        let ppid = unsafe { libc::getppid() };
        println!("child process: pid={pid}, parent process pid={ppid}");
    } else if ret > 0 {
        let pid = unsafe { libc::getpid() };
        println!("parent process: pid={pid}, child process pid={ret}");
    }
}
