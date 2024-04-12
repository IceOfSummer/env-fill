pub mod replacer;
pub mod result;
pub mod schema;

use std::io::Read;
use std::path::{Path};
use std::fs;
use std::error::Error;
use glob::glob;

pub fn run(config: schema::Config) -> Result<(), Box<dyn Error>> {
    let dest_absolute_path = to_abs_path(&config.files.dest)?;
    let pwd_absolute_path = to_abs_path(&config.files.pwd)?;
    
    for file_pattern in config.files.include {
        let path = Path::new(&pwd_absolute_path);
        let path = path.join(&file_pattern);
        match glob(path.to_str().unwrap()) {
            Err(e) => {
                return Err(Box::new(result::UnExpectedError::Argument(format!("Error while parse pattern: {}. reason: {}", file_pattern, e))));
            },
            Ok(paths) => {
                for path in paths {
                    let path = path?;
                    let path = path.to_str().unwrap().to_string();
                    let content = read_file(&path)?;
                    let content = replacer::replace_content(&content, &config.env);
                    let final_path = path.replace(&pwd_absolute_path, &dest_absolute_path);
                    println!("save file {}", final_path);
                    save_file(&final_path, &content)?;
                }        
            }
        }
    }
    Ok(())
}

fn to_abs_path(path: &str) -> Result<String, result::UnExpectedError> {
    let path = match path.is_empty() {
        true => ".",
        false => path
    };
    let str = fs::canonicalize(path)?;
    let str = str.to_str();
    match str {
        None => Err(result::UnExpectedError::Argument(format!("无法解析路径: {}", path))),
        Some(val) => Ok(val.to_string())
    }
    
}

///
/// 读取文件内容
///
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    fs::File::read_to_string(&mut file, &mut contents)?;
    Ok(contents)
}

///
/// 保存文件
///
fn save_file(path: &str, content: &str) -> Result<(), std::io::Error> {
    fs::write(path, content)?;
    Ok(())
}

#[cfg(test)]
mod test {

    use std::env;
    use crate::to_abs_path;

    #[test]
    pub fn test_to_abs_path() {
        let dir = env::current_dir().unwrap().to_str().unwrap().to_string();

        assert_eq!(to_abs_path(".").unwrap(), dir);
        assert_eq!(to_abs_path("").unwrap(), dir);
    }
    
}