mod schema;
mod replacer;

use std::fs::File;
use std::fs;
use std::io::{Read, Write};
use glob::glob;
use crate::replacer::replace_content;
use std::path::{Path};

fn main() {
    let config: schema::Config = load_config_file();

    let dest_absolute_path = fs::canonicalize(&config.files.dest).unwrap().to_str().unwrap().to_string();
    let pwd_absolute_path = fs::canonicalize(&config.files.pwd).unwrap().to_str().unwrap().to_string();
    for file_pattern in config.files.include {
        let path = Path::new(&pwd_absolute_path);
        let path = path.join(&file_pattern);
        match glob(path.to_str().unwrap()) {
            Err(e) => panic!("Invalid pattern: {}, reason: {}.", file_pattern, e),
            Ok(paths) => {
                for path in paths {
                    let p = path.ok().unwrap().into_os_string().into_string().ok().unwrap();
                    let content = read_file(&p);
                    let content = replace_content(&content, &config.env);
                    let final_path = p.replace(&pwd_absolute_path, &dest_absolute_path);
                    println!("save file {}", final_path);
                    save_file(&final_path, &content)
                }        
            }
        }
    }
}


///
/// 读取文件内容
///
fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok();
    contents
}

/// 
/// 从文件中加载配置文件
///
fn load_config_file() -> schema::Config {
    schema::parse_content(&read_file("env.toml"))
}

///
/// 保存文件
///
fn save_file(path: &str, content: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).ok();
}