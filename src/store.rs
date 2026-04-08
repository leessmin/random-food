use std::{
    fs::{self, File},
    io::Write,
};

const README_MD: &'static str = "README.md";
const VERSION_FILE: &'static str = "VERSION";
const ENV_APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub enum StoreName {
    README,
    VERSION,
}

impl StoreName {
    fn as_str(&self) -> &'static str {
        match self {
            StoreName::README => README_MD,
            StoreName::VERSION => VERSION_FILE,
        }
    }
}

fn get_store_path(store_name: StoreName) -> std::io::Result<std::path::PathBuf> {
    let file_path = dirs::data_local_dir().unwrap();
    let file_path = file_path.join(ENV_APP_NAME).join(store_name.as_str());

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    Ok(file_path)
}

// 判断是否存在储存文件
pub fn judge_store_exists(store_name: StoreName) -> bool {
    let file_path = get_store_path(store_name);
    if let Ok(path) = file_path {
        path.exists()
    } else {
        false
    }
}

// 储存文件
pub fn store_write(store_name: StoreName, buf: &[u8]) -> std::io::Result<()> {
    let file_path = get_store_path(store_name)?;

    let mut file = File::create(&file_path)?;
    file.write_all(buf)?;

    Ok(())
}

// 读取文件
pub fn store_read(store_name: StoreName) -> std::io::Result<String> {
    let file_path = get_store_path(store_name)?;

    let content = fs::read_to_string(&file_path)?;

    Ok(content)
}
