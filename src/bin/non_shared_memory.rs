fn main() {
    let mut data = 1000;

    println!("data value before spawning child process: {data}");
    let pid: u32 = unsafe { libc::fork() }
        .try_into()
        .expect("failed to fork process");
    #[allow(unused_assignments)]
    if pid == 0 {
        data *= 2;
        std::process::exit(0);
    }

    let mut status = 0;
    let _ = unsafe { libc::waitpid(pid as i32, &mut status, 0) };
    assert!(
        libc::WIFEXITED(status),
        "child process `{pid}` did not terminate normally"
    );

    println!("data value after child process finished: {data}");
}
