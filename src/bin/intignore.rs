fn main() {
    let _ = unsafe { libc::signal(libc::SIGINT, libc::SIG_IGN) };
    #[allow(clippy::empty_loop)]
    loop {}
}
