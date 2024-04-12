use std::process;
use env_fill::schema;

fn main() {
    let config: schema::Config = load_config_file();
    if let Err(e) = env_fill::run(config) {
        println!("Application error!\n{e}");
        process::exit(1);
    }
}


/// 
/// 从文件中加载配置文件
///
fn load_config_file() -> schema::Config {
    schema::parse_content(&env_fill::read_file("env.toml").expect("配置文件不存在"))
}

