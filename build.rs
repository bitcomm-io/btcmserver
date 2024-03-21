use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 获取当前工作目录
    let current_dir = env::current_dir().unwrap();

    // 指定 admin 目录路径
    let source_dir = current_dir.join("admin");

    // 指定目标目录路径（根据编译模式选择）
    let target_dir = if env::var("PROFILE").unwrap() == "release" {
        current_dir.join("target").join("release").join("admin")
    } else {
        current_dir.join("target").join("debug").join("admin")
    };

    // 将 admin 目录复制到目标目录
    copy_dir(&source_dir, &target_dir).unwrap();
}

fn copy_dir(source: &Path, target: &Path) -> std::io::Result<()> {
    // 创建目标目录
    if !target.exists() {
        fs::create_dir_all(&target)?;
    }

    // 遍历源目录中的所有条目
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let file_name = source_path.file_name().unwrap();
        let target_path = target.join(file_name);

        // 如果是目录，则递归复制
        if source_path.is_dir() {
            copy_dir(&source_path, &target_path)?;
        } else {
            // 如果是文件，则复制到目标目录
            fs::copy(&source_path, &target_path)?;
        }
    }
    Ok(())
}
