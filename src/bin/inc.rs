const FILE_NAME: &str = "count";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let value: u32 = std::fs::read_to_string(FILE_NAME)?.trim().parse()?;
    std::fs::write(FILE_NAME, format!("{}\n", value + 1).as_bytes())?;
    Ok(())
}
