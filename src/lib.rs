use std::fs;
use std::time::UNIX_EPOCH;

pub fn get_timestamp(path: &str) -> std::io::Result<u64> {
    let metadata = fs::metadata(path)?;
    let modified_time = metadata.modified()?;
    let duration_since_epoch = modified_time.duration_since(UNIX_EPOCH)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Time went backwards"))?;
    Ok(duration_since_epoch.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_timestamp() {
        let path = "./src/lib.rs";
        let timestamp = get_timestamp(path).unwrap();
        println!("Timestamp: {}", timestamp);
        // 这里你可以添加一些断言来验证结果
    }
}