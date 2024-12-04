use rayon::prelude::*;
use std::fs::{self, File};
use std::io::{Error, Write};
use std::path::Path;

use ncmdump::Ncmdump;

fn main() -> Result<(), Error> {
    // 创建 unlock 文件夹（如果不存在）
    fs::create_dir_all("unlock")?;

    // 读取当前目录下的所有文件
    let entries: Vec<_> = fs::read_dir(".")?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let path = entry.path();
            path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("ncm")
        })
        .collect();

    // 并行处理每个 .ncm 文件
    entries.par_iter().for_each(|entry| {
        if let Err(e) = process_ncm_file(entry) {
            eprintln!("Error processing file {:?}: {}", entry.path(), e);
        }
    });

    Ok(())
}

fn process_ncm_file(entry: &fs::DirEntry) -> Result<(), Error> {
    let path = entry.path();
    let file = File::open(&path)?;
    let mut ncm = Ncmdump::from_reader(file).expect("Can't create dump");
    let music = ncm.get_data().expect("Can't get data");

    // 获取文件名（不包括扩展名）
    let file_stem = path.file_stem().unwrap();
    // 构建输出文件路径，添加 .flac 扩展名
    let output_path = Path::new("unlock").join(format!("{}.flac", file_stem.to_string_lossy()));

    let mut target = File::options().create(true).write(true).open(output_path)?;
    target.write_all(&music)?;

    Ok(())
}
