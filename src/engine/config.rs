use std::path::Path;
use yaml_rust::yaml::{Yaml, YamlLoader};
use engine::util::file;

// define config values
#[derive(Debug, Clone)]
pub struct Config {
    yaml: Yaml,
    window_width: u32,
    window_height: u32,
    fullscreen: bool,
    vsync: bool,
    title: String,
}

// path to the config file
pub const CONFIG_FILE_PATH: &'static str = "resources/config.yaml";

impl Config {

    pub fn load() -> Self {
        // load the default config set and make the mutable
        let mut config : Config = Default::default();
        // if a config file exists load it and replace then inthe
        if Path::new(&CONFIG_FILE_PATH).exists() {
            // get yaml file content
            let yaml_file = file::read_file(&CONFIG_FILE_PATH); 
            // parse file content as `Yaml` struct into `Config`
            config.yaml = YamlLoader::load_from_str(&yaml_file).unwrap().pop().unwrap();

            // override the default settings
            config.fullscreen       = config.yaml["fullscreen"]     .as_bool()  .unwrap();
            config.window_width     = config.yaml["window_width"]   .as_i64()   .unwrap() as u32;
            config.window_height    = config.yaml["window_height"]  .as_bool()  .unwrap() as u32;
            config.title            = config.yaml["title"]          .as_str()   .unwrap().to_string();
        }
        // return `Config`
        config
    }


}

impl Default for Config {
    fn default() -> Self{
        Config{
            yaml: Yaml::from_str(""),
            window_height: 600,
            window_width: 800,
            fullscreen: false,
            vsync: false,
            title: String::new()
        }
    }
}
