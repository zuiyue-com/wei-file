fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "./src/lib.rs";
    let timestamp = wei_file::get_timestamp(path)?;
    println!("Timestamp: {}", timestamp);
    Ok(())
}