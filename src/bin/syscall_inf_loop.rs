fn main() {
    loop {
        let _ = unsafe { libc::getppid() };
    }
}
