const LOCK_FILE: &str = "lock";
const COUNTER_FILE: &str = "count";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        if !std::fs::exists(LOCK_FILE)? {
            break;
        }
    }

    let _ = std::fs::File::create(LOCK_FILE)?;

    let value: u32 = std::fs::read_to_string(COUNTER_FILE)?.trim().parse()?;
    std::fs::write(COUNTER_FILE, format!("{}\n", value + 1).as_bytes())?;

    std::fs::remove_file(LOCK_FILE)?;

    Ok(())
}
