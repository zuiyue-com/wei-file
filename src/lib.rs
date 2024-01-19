use std::fs;
use std::time::UNIX_EPOCH;

pub fn get_timestamp(path: &str) -> std::io::Result<u64> {
    let metadata = fs::metadata(path)?;
    let modified_time = metadata.modified()?;
    let duration_since_epoch = modified_time.duration_since(UNIX_EPOCH)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Time went backwards"))?;
    Ok(duration_since_epoch.as_secs())
}

use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;

pub fn xz_decompress(lzma_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 打开一个 lzma 压缩文件
    let lzma_file = File::open(lzma_file_path)?;
    let path = Path::new(lzma_file_path);
    // 获取lzma_file的目录
    let lzma_file_dir = path.parent().ok_or("No parent directory")?;
    // 获取lzma_file的文件名
    let lzma_file_name = path.file_name().ok_or("No file name")?.to_str().ok_or("Cannot convert to str")?.replace(".xz", "");
    let mut lzma_reader = BufReader::new(lzma_file);

    // 获取file的文件名
    let tar_file_name = lzma_file_name.clone().replace(".xz", "");

    // 创建一个新的 tar 文件
    let tar_file = File::create(lzma_file_dir.join(lzma_file_name))?;
    let mut tar_writer = BufWriter::new(tar_file);

    // 解压缩 lzma 文件到 tar 文件
    lzma_rs::xz_decompress(&mut lzma_reader, &mut tar_writer)?;

    // 解压 tar 文件到 lzma_file_dir 目录
    let tar_file = File::open(lzma_file_dir.join(tar_file_name))?;
    let mut archive = tar::Archive::new(BufReader::new(tar_file));
    archive.unpack(lzma_file_dir)?;

    Ok(())
}


pub fn xz_compress(tar_dir_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tar_dir = Path::new(tar_dir_path);
    let tar_parent_dir = tar_dir.parent().ok_or("No parent directory")?;
    // 我要分解这个 tar_dir_path 的目录，如何分解？
    let tar_dir = tar_dir.file_name().ok_or("No file name")?.to_str().ok_or("Cannot convert to str")?;

    // 打开一个目录，使用wsl tar 并压缩目录下的所有文件
    #[cfg(target_os = "windows")]
    let data = std::process::Command::new("wsl")
        .arg("tar")
        .arg("-cJf")
        .arg(format!("{}.tar.xz", tar_dir))
        .arg(tar_dir)
        .current_dir(tar_parent_dir)
        .output().expect("failed to execute process");

    #[cfg(target_os = "linux")]
    let data = std::process::Command::new("tar")
        .arg("-cJf")
        .arg(format!("{}.tar.xz", tar_dir))
        .arg(tar_dir)
        .current_dir(tar_parent_dir)
        .output().expect("failed to execute process");

    if !data.status.success() {
        let data = format!("{:?}", data);
        return std::result::Result::Err(data.into());
    }

    Ok(())
}