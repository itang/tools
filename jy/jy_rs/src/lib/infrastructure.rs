//!infrastructure

use std::fs;
use std::path::{Path, PathBuf};

use toml::Value;

use crate::constants;
use crate::domain::{Browser, ConfigProvider, Url};

pub struct BrowserImpl;

impl Browser for BrowserImpl {
    fn browser_single_url(&self, url: &Url) -> anyhow::Result<()> {
        Ok(webbrowser::open(url)?) // Deref coercion
        //If T implements Deref<Target = U>, and v is a value of type T, then:
        //    In immutable contexts, *v (where T is neither a reference nor a raw pointer) is equivalent to *Deref::deref(&v).
        //    Values of type &T are coerced to values of type &U
        //    T implicitly implements all the methods of the type U which take the &self receiver.
    }
}

pub struct ConfigProviderImpl {}

impl ConfigProvider for ConfigProviderImpl {
    fn get_config(&self, path: Option<PathBuf>) -> anyhow::Result<Value> {
        let config = match path {
            Some(path) => {
                println!("INFO: 配置路径:{:?}", path);
                match get_config(&path) {
                    Ok(content) => content,
                    Err(e) => panic!("WARN: 尝试从配置路径加载文件失败 {:?}, error: {}", path, e),
                }
            },
            None => {
                let home_config_dir = dirs::home_dir().expect("get home dir").join(".jy");
                println!("INFO: 未指定要加载配置文件, 尝试从{:?}加载", home_config_dir);

                let home_config_path = home_config_dir.join(constants::DEFAULT_FILE_NAME);
                if home_config_path.exists() {
                    println!("INFO: 默认配置文件存在 {:?}", home_config_path);
                    get_config(home_config_path)?
                } else {
                    fs::create_dir(home_config_dir).expect("create dir");
                    println!("WARN: 默认配置文件不存在, 使用默认配置列表创建{:?}...", home_config_path);
                    fs::write(home_config_path, constants::DEFAULT_CONFIG).expect("write file");
                    constants::DEFAULT_CONFIG.into()
                }
            },
        };

        Ok(config)
    }
}

/// get content from path
fn get_config<P>(config_path: P) -> anyhow::Result<Value>
where
    P: AsRef<Path>,
{
    let config_path: &Path = config_path.as_ref();
    println!("INFO: read from config file: {:?}", config_path);

    let result = fs::read_to_string(config_path)?.parse::<Value>();

    Ok(result?)
}
