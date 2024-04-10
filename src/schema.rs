use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Files {
    /// 包含哪些文件
    pub include: Vec<String>,
    /// 输出到哪个路径下
    pub dest: String,
    /// 当前路径
    pub pwd: String
}

#[derive(Deserialize, Debug)]
pub struct Config {

    pub env: HashMap<String, String>,

    pub files: Files

}

///
/// 将字符串转换成配置文件
///
pub fn parse_content(content: &String) -> Config {
    let config: Config = toml::from_str(content).unwrap();
    config
}