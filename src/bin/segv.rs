fn main() {
    let p: *mut i32 = std::ptr::null_mut();
    println!("before illegal memory access");
    unsafe { *p = 0 };
    println!("after illegal memory access");
}
