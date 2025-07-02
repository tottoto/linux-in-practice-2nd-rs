const SIZE: usize = 10000000;

fn main() -> std::io::Result<()> {
    let mut free = std::process::Command::new("free");

    println!("system memory usage before memory allocation");
    println!("{}", String::from_utf8_lossy(&free.output()?.stdout));

    let _array: Vec<u8> = std::iter::repeat_n(0, SIZE).collect();

    println!("system memory usage after memory allocation");
    println!("{}", String::from_utf8_lossy(&free.output()?.stdout));

    Ok(())
}
